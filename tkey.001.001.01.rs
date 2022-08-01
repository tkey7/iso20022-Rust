// TKEY7 iso20022

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;


// document 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct document {
	#[serde(rename = "Document")]
	pub document: Document,
}


// ExternalFinancialInstitutionIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// Max35Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// ISODateTime 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: u8,
}


// BatchBookingIndicator 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BatchBookingIndicator {
	#[serde(rename = "BatchBookingIndicator")]
	pub batch_booking_indicator: bool,
}


// Max15NumericText 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max15NumericText {
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// DecimalNumber 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// ActiveCurrencyAndAmountSimpleType 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ISODate 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: u8,
}


// SettlementMethod1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementMethod1Code {
	#[serde(rename = "SettlementMethod1Code")]
	pub settlement_method1_code: String,
}


// ExternalCashClearingSystem1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
}


// IBAN2007Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// Max34Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// ExternalAccountIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ActiveOrHistoricCurrencyCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// Max70Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// SettlementInstruction4 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementInstruction4 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[serde(rename = "SttlmAcct")]
	pub sttlm_acct: CashAccount24,
	#[serde(rename = "ClrSys")]
	pub clr_sys: ClearingSystemIdentification3Choice,
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification5,
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: CashAccount24,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification5,
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: CashAccount24,
	#[serde(rename = "ThrdRmbrsmntAgt")]
	pub thrd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification5,
	#[serde(rename = "ThrdRmbrsmntAgtAcct")]
	pub thrd_rmbrsmnt_agt_acct: CashAccount24,
}


// ClearingSystemIdentification3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CashAccount24 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccount24 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp")]
	pub tp: CashAccountType2Choice,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "Nm")]
	pub nm: String,
}


// AccountIdentification4Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: String,
	#[serde(rename = "Othr")]
	pub othr: GenericAccountIdentification1,
}


// GenericAccountIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: AccountSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// AccountSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CashAccountType2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// BranchAndFinancialInstitutionIdentification5 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification5 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification8,
	#[serde(rename = "BrnchId")]
	pub brnch_id: BranchData2,
}


// FinancialInstitutionIdentification8 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionIdentification8 {
	#[serde(rename = "BICFI")]
	pub bicfi: String,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: ClearingSystemMemberIdentification2,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress6,
	#[serde(rename = "Othr")]
	pub othr: GenericFinancialIdentification1,
}


// BICFIIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BICFIIdentifier {
	#[serde(rename = "BICFIIdentifier")]
	pub bicfi_identifier: String,
}


// ClearingSystemMemberIdentification2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: ClearingSystemIdentification2Choice,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// ClearingSystemIdentification2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalClearingSystemIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// Max140Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// PostalAddress6 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostalAddress6 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: String,
	#[serde(rename = "Dept")]
	pub dept: String,
	#[serde(rename = "SubDept")]
	pub sub_dept: String,
	#[serde(rename = "StrtNm")]
	pub strt_nm: String,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: String,
	#[serde(rename = "PstCd")]
	pub pst_cd: String,
	#[serde(rename = "TwnNm")]
	pub twn_nm: String,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: String,
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "AdrLine")]
	pub adr_line: Vec<String>,
}


// AddressType2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// Max16Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// CountryCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// FinancialIdentificationSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// GenericFinancialIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: FinancialIdentificationSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// BranchData2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchData2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress6,
}


// PaymentTypeInformation21 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentTypeInformation21 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: String,
	#[serde(rename = "ClrChanl")]
	pub clr_chanl: String,
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: ServiceLevel8Choice,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: LocalInstrument2Choice,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: CategoryPurpose1Choice,
}


// Priority2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Priority2Code {
	#[serde(rename = "Priority2Code")]
	pub priority2_code: String,
}


// ClearingChannel2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingChannel2Code {
	#[serde(rename = "ClearingChannel2Code")]
	pub clearing_channel2_code: String,
}


// ServiceLevel8Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalServiceLevel1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "ExternalServiceLevel1Code")]
	pub external_service_level1_code: String,
}


// LocalInstrument2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalLocalInstrument1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "ExternalLocalInstrument1Code")]
	pub external_local_instrument1_code: String,
}


// CategoryPurpose1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalCategoryPurpose1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "ExternalCategoryPurpose1Code")]
	pub external_category_purpose1_code: String,
}


// GroupHeader70 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GroupHeader70 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "BtchBookg")]
	pub btch_bookg: bool,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: String,
	#[serde(rename = "CtrlSum")]
	pub ctrl_sum: f64,
	#[serde(rename = "TtlIntrBkSttlmAmt")]
	pub ttl_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction4,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: PaymentTypeInformation21,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification5,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: BranchAndFinancialInstitutionIdentification5,
}


// CustomerIdentificationStatusNotificationV01 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CustomerIdentificationStatusNotificationV01 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader70,
	#[serde(rename = "AcctInfAndIdSts")]
	pub acct_inf_and_id_sts: Vec<AccountInformationAndIdentificationStatus1>,
}


// AccountInformationAndIdentificationStatus1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountInformationAndIdentificationStatus1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "AcctIdInf")]
	pub acct_id_inf: AccountIdentificationInformation1,
	#[serde(rename = "CstmrIdInf")]
	pub cstmr_id_inf: CustomerIdentificationInformation1,
}


// AccountIdentificationInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountIdentificationInformation1 {
	#[serde(rename = "AdrsngId")]
	pub adrsng_id: String,
	#[serde(rename = "SttlmAcctId")]
	pub sttlm_acct_id: String,
	#[serde(rename = "NoteTp")]
	pub note_tp: String,
	#[serde(rename = "Note")]
	pub note: String,
	#[serde(rename = "CstmrAcctIdSts")]
	pub cstmr_acct_id_sts: String,
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification3,
}


// CustomerIdentificationInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CustomerIdentificationInformation1 {
	#[serde(rename = "KYCSts")]
	pub kyc_sts: String,
	#[serde(rename = "AMLSts")]
	pub aml_sts: String,
	#[serde(rename = "PIIInf")]
	pub pii_inf: String,
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification3,
}


// PaymentIdentification3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentIdentification3 {
	#[serde(rename = "InstrId")]
	pub instr_id: String,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: String,
	#[serde(rename = "TxId")]
	pub tx_id: String,
	#[serde(rename = "ClrSysRef")]
	pub clr_sys_ref: String,
}


// NoteType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoteType1 {
	#[serde(rename = "NoteType1")]
	pub note_type1: String,
}


// Note1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Note1 {
	#[serde(rename = "Note1")]
	pub note1: String,
}


// CustomerAccountIdentificationStatus1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CustomerAccountIdentificationStatus1 {
	#[serde(rename = "CustomerAccountIdentificationStatus1")]
	pub customer_account_identification_status1: String,
}


// PersonallyIdentifiableInformationInfo1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonallyIdentifiableInformationInfo1 {
	#[serde(rename = "PersonallyIdentifiableInformationInfo1")]
	pub personally_identifiable_information_info1: String,
}


// KYCStatus1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct KYCStatus1 {
	#[serde(rename = "KYCStatus1")]
	pub kyc_status1: String,
}


// AMLStatus1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AMLStatus1 {
	#[serde(rename = "AMLStatus1")]
	pub aml_status1: String,
}


// addressing_identification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct addressing_identification1 {
	#[serde(rename = "AddressingIdentification1")]
	pub addressing_identification1: String,
}


// settlement_account_identification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct settlement_account_identification1 {
	#[serde(rename = "SettlementAccountIdentification1")]
	pub settlement_account_identification1: String,
}


// DateTimeString 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateTimeString {
	#[serde(rename = "DateTimeString")]
	pub date_time_string: String,
}


// DateString 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateString {
	#[serde(rename = "DateString")]
	pub date_string: String,
}
