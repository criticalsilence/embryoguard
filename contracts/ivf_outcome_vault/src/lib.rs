use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, NearToken, PanicOnDefault, Promise};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum VaultStage {
    Funded,                  // Patient deposited via NEAR Intents (100% locked)
    OpuCompleted,            // Milestone 1 (20% released to clinic after Oocyte Pick-Up)
    BlastocystVerified,      // Milestone 2 (30% released after Embryologist signs Day-5 Gardner report)
    OutcomeSuccessSettled,   // Milestone 3A (Remaining 50% released on confirmed Clinical Pregnancy)
    OutcomeRefundTriggered,  // Milestone 3B (Automated Shared-Risk Refund sent back to Patient via NEAR Intents)
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmbryologyLabReport {
    pub retrieved_oocytes: u8,
    pub mature_mii_oocytes: u8,
    pub fertilized_2pn_icsi: u8,
    pub day5_blastocysts: u8,
    pub top_gardner_grade: String, // e.g., "5AA", "4AA", "4AB"
    pub pgt_a_euploid_count: u8,
    pub embryoscope_timelapse_hash: String, // SHA-256 hash of laboratory time-lapse video/report
    pub verified_by_embryologist: AccountId,
    pub verified_at_ms: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PatientAssuranceCase {
    pub case_id: String,
    pub patient_account: AccountId,
    pub patient_payout_intent_chain: String, // e.g., "near:mainnet", "arb:usdc", "sol:usdc"
    pub clinic_account: AccountId,
    pub clinic_id: String,
    pub total_deposit_yocto: u128,
    pub released_to_clinic_yocto: u128,
    pub refunded_to_patient_yocto: u128,
    pub refund_guarantee_bps: u16, // e.g., 5000 = 50% of total package refunded if cycle fails
    pub predicted_success_bps: u16, // e.g., 6400 = 64.0% calculated by Private AI + Embryologist model
    pub stage: VaultStage,
    pub lab_report: Option<EmbryologyLabReport>,
    pub created_at_ms: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EmbryoGuardVault {
    pub lead_embryologist_oracle: AccountId,
    pub intents_solver_bridge: AccountId,
    pub cases: UnorderedMap<String, PatientAssuranceCase>,
    pub total_locked_yocto: u128,
    pub total_refunded_yocto: u128,
}

#[near_bindgen]
impl EmbryoGuardVault {
    #[init]
    pub fn new(lead_embryologist_oracle: AccountId, intents_solver_bridge: AccountId) -> Self {
        Self {
            lead_embryologist_oracle,
            intents_solver_bridge,
            cases: UnorderedMap::new(b"c"),
            total_locked_yocto: 0,
            total_refunded_yocto: 0,
        }
    }

    /// Step 1: Patient funds an Outcome Assurance IVF Vault (directly or settled via NEAR Intents)
    #[payable]
    pub fn create_assurance_case(
        &mut self,
        case_id: String,
        clinic_account: AccountId,
        clinic_id: String,
        patient_payout_intent_chain: String,
        refund_guarantee_bps: u16,
        predicted_success_bps: u16,
    ) -> PatientAssuranceCase {
        let deposit = env::attached_deposit().as_yoctonear();
        assert!(deposit > 0, "Deposit must be greater than 0");
        assert!(
            refund_guarantee_bps <= 5000,
            "Max milestone-3 outcome refund guarantee is 5000 bps (50% of total vault)"
        );
        assert!(
            self.cases.get(&case_id).is_none(),
            "Case ID already exists in EmbryoGuard Vault"
        );

        let patient_case = PatientAssuranceCase {
            case_id: case_id.clone(),
            patient_account: env::predecessor_account_id(),
            patient_payout_intent_chain,
            clinic_account,
            clinic_id,
            total_deposit_yocto: deposit,
            released_to_clinic_yocto: 0,
            refunded_to_patient_yocto: 0,
            refund_guarantee_bps,
            predicted_success_bps,
            stage: VaultStage::Funded,
            lab_report: None,
            created_at_ms: env::block_timestamp_ms(),
        };

        self.cases.insert(&case_id, &patient_case);
        self.total_locked_yocto += deposit;

        env::log_str(&format!(
            "EVENT_JSON:{{\"standard\":\"embryoguard\",\"version\":\"1.0.0\",\"event\":\"case_funded\",\"data\":{{\"case_id\":\"{}\",\"deposit\":\"{}\"}}}}",
            case_id, deposit
        ));

        patient_case
    }

    /// Step 2 (Milestone 1 - 20%): Confirm OPU (Oocyte Pick-Up) & Release 20% to Partner Clinic
    pub fn confirm_opu_milestone(&mut self, case_id: String) -> PatientAssuranceCase {
        self.assert_embryologist_or_patient(&case_id);
        let mut c = self.cases.get(&case_id).expect("Case not found");
        assert_eq!(c.stage, VaultStage::Funded, "Case must be in Funded stage");

        let milestone_1_amount = (c.total_deposit_yocto * 20) / 100;
        c.released_to_clinic_yocto += milestone_1_amount;
        c.stage = VaultStage::OpuCompleted;
        self.cases.insert(&case_id, &c);

        Promise::new(c.clinic_account.clone()).transfer(NearToken::from_yoctonear(milestone_1_amount));
        c
    }

    /// Step 3 (Milestone 2 - 30%): Lead Embryologist signs transparent Day-5 Blastocyst Report on-chain
    pub fn attest_blastocyst_report(
        &mut self,
        case_id: String,
        retrieved_oocytes: u8,
        mature_mii_oocytes: u8,
        fertilized_2pn_icsi: u8,
        day5_blastocysts: u8,
        top_gardner_grade: String,
        pgt_a_euploid_count: u8,
        embryoscope_timelapse_hash: String,
    ) -> PatientAssuranceCase {
        assert_eq!(
            env::predecessor_account_id(),
            self.lead_embryologist_oracle,
            "Only the certified Lead Embryologist Oracle can sign laboratory blastocyst reports"
        );

        let mut c = self.cases.get(&case_id).expect("Case not found");
        assert_eq!(
            c.stage,
            VaultStage::OpuCompleted,
            "OPU milestone must be completed before Day-5 Blastocyst verification"
        );

        let report = EmbryologyLabReport {
            retrieved_oocytes,
            mature_mii_oocytes,
            fertilized_2pn_icsi,
            day5_blastocysts,
            top_gardner_grade: top_gardner_grade.clone(),
            pgt_a_euploid_count,
            embryoscope_timelapse_hash,
            verified_by_embryologist: env::predecessor_account_id(),
            verified_at_ms: env::block_timestamp_ms(),
        };

        let milestone_2_amount = (c.total_deposit_yocto * 30) / 100;
        c.released_to_clinic_yocto += milestone_2_amount;
        c.lab_report = Some(report);
        c.stage = VaultStage::BlastocystVerified;
        self.cases.insert(&case_id, &c);

        Promise::new(c.clinic_account.clone()).transfer(NearToken::from_yoctonear(milestone_2_amount));

        env::log_str(&format!(
            "EVENT_JSON:{{\"standard\":\"embryoguard\",\"version\":\"1.0.0\",\"event\":\"blastocyst_verified\",\"data\":{{\"case_id\":\"{}\",\"top_grade\":\"{}\",\"blastocysts\":{}}}}}",
            case_id, top_gardner_grade, day5_blastocysts
        ));

        c
    }

    /// Step 4 (Milestone 3 - 50% Outcome Assurance):
    /// If clinical pregnancy (Beta-hCG + clinical ultrasound) is confirmed -> release remaining 50% to clinic.
    /// If negative -> automatically refund guaranteed portion back to patient via NEAR Intents!
    pub fn resolve_clinical_outcome(
        &mut self,
        case_id: String,
        pregnancy_confirmed: bool,
    ) -> PatientAssuranceCase {
        assert_eq!(
            env::predecessor_account_id(),
            self.lead_embryologist_oracle,
            "Only Lead Embryologist Oracle can attest clinical outcome"
        );

        let mut c = self.cases.get(&case_id).expect("Case not found");
        assert_eq!(
            c.stage,
            VaultStage::BlastocystVerified,
            "Blastocyst verification must be completed before outcome resolution"
        );

        let remaining_yocto = c.total_deposit_yocto - c.released_to_clinic_yocto;

        if pregnancy_confirmed {
            c.released_to_clinic_yocto += remaining_yocto;
            c.stage = VaultStage::OutcomeSuccessSettled;
            self.cases.insert(&case_id, &c);
            Promise::new(c.clinic_account.clone()).transfer(NearToken::from_yoctonear(remaining_yocto));
        } else {
            let refund_amount = (c.total_deposit_yocto * (c.refund_guarantee_bps as u128)) / 10000;
            let actual_refund = refund_amount.min(remaining_yocto);
            let clinic_remainder = remaining_yocto - actual_refund;

            c.refunded_to_patient_yocto = actual_refund;
            c.released_to_clinic_yocto += clinic_remainder;
            c.stage = VaultStage::OutcomeRefundTriggered;
            self.total_refunded_yocto += actual_refund;
            self.cases.insert(&case_id, &c);

            if actual_refund > 0 {
                Promise::new(c.patient_account.clone()).transfer(NearToken::from_yoctonear(actual_refund));
            }
            if clinic_remainder > 0 {
                Promise::new(c.clinic_account.clone()).transfer(NearToken::from_yoctonear(clinic_remainder));
            }
        }

        c
    }

    pub fn get_case(&self, case_id: String) -> Option<PatientAssuranceCase> {
        self.cases.get(&case_id)
    }

    fn assert_embryologist_or_patient(&self, case_id: &String) {
        let c = self.cases.get(case_id).expect("Case not found");
        let caller = env::predecessor_account_id();
        assert!(
            caller == self.lead_embryologist_oracle || caller == c.patient_account,
            "Caller must be Lead Embryologist Oracle or Patient"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[test]
    fn test_full_ivf_outcome_refund_flow() {
        let embryologist: AccountId = "caner-embryologist.near".parse().unwrap();
        let solver: AccountId = "intents-solver.near".parse().unwrap();
        let patient: AccountId = "alice-patient.near".parse().unwrap();
        let clinic: AccountId = "istanbul-ivf-clinic.near".parse().unwrap();

        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(patient.clone());
        context.attached_deposit(NearToken::from_near(100));
        testing_env!(context.build());

        let mut contract = EmbryoGuardVault::new(embryologist.clone(), solver);
        let c = contract.create_assurance_case(
            "IVF-2026-001".to_string(),
            clinic,
            "IST-ART-01".to_string(),
            "arb:usdc".to_string(),
            5000, // 50% refund guarantee
            6200, // 62% predicted success
        );
        assert_eq!(c.stage, VaultStage::Funded);

        // Milestone 1: OPU
        let c = contract.confirm_opu_milestone("IVF-2026-001".to_string());
        assert_eq!(c.stage, VaultStage::OpuCompleted);

        // Milestone 2: Embryologist Blastocyst Verification
        context.predecessor_account_id(embryologist.clone());
        testing_env!(context.build());
        let c = contract.attest_blastocyst_report(
            "IVF-2026-001".to_string(),
            12,
            10,
            8,
            4,
            "5AA".to_string(),
            2,
            "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string(),
        );
        assert_eq!(c.stage, VaultStage::BlastocystVerified);

        // Milestone 3: Negative pregnancy triggers automated 50% refund
        let c = contract.resolve_clinical_outcome("IVF-2026-001".to_string(), false);
        assert_eq!(c.stage, VaultStage::OutcomeRefundTriggered);
        assert_eq!(c.refunded_to_patient_yocto, NearToken::from_near(50).as_yoctonear());
    }
}
