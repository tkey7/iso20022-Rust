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


// SettlementMethod1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementMethod1Code {
	#[serde(rename = "SettlementMethod1Code")]
	pub settlement_method1_code: String,
}


// ClearingSystemIdentification3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalCashClearingSystem1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
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


// IBAN2007Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
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


// Max34Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// AccountSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalAccountIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// CashAccountType2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
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


// FinancialIdentificationSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalFinancialInstitutionIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
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
	pub cre_dt_tm: u8,
	#[serde(rename = "BtchBookg")]
	pub btch_bookg: bool,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: String,
	#[serde(rename = "CtrlSum")]
	pub ctrl_sum: f64,
	#[serde(rename = "TtlIntrBkSttlmAmt")]
	pub ttl_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: u8,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction4,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: PaymentTypeInformation21,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification5,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: BranchAndFinancialInstitutionIdentification5,
}


// TransactionSettlementNotificationV01 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TransactionSettlementNotificationV01 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader70,
	#[serde(rename = "TxSttlmInf")]
	pub tx_sttlm_inf: Vec<TransactionSettlementInformation1>,
}


// TransactionSettlementInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TransactionSettlementInformation1 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: OriginalGroupInformation3,
	#[serde(rename = "TxInf")]
	pub tx_inf: Vec<PaymentTransactionInformation2>,
}


// PaymentTransactionInformation2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentTransactionInformation2 {
	#[serde(rename = "SttlmId")]
	pub sttlm_id: String,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: String,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: String,
	#[serde(rename = "OrgnlTxId")]
	pub orgnl_tx_id: String,
	#[serde(rename = "ActlSttlmAmt")]
	pub actl_sttlm_amt: CurrencyAndAmount,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: OriginalTransactionReference1,
}


// CurrencyAndAmount 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(flatten)]
	pub f64: f64,
}


// OriginalGroupInformation3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OriginalGroupInformation3 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: u8,
}


// CurrencyAndAmountSimpleType 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CurrencyAndAmountSimpleType {
	#[serde(rename = "CurrencyAndAmount_SimpleType")]
	pub currency_and_amount_simple_type: f64,
}


// CurrencyCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CurrencyCode {
	#[serde(rename = "CurrencyCode")]
	pub currency_code: String,
}


// OriginalTransactionReference1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OriginalTransactionReference1 {
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: CurrencyAndAmount,
	#[serde(rename = "Amt")]
	pub amt: AmountType2Choice,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: u8,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: u8,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: u8,
	#[serde(rename = "CdtrSchmeId")]
	pub cdtr_schme_id: PartyIdentification8,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInformation3,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: PaymentTypeInformation6,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: String,
	#[serde(rename = "MndtRltdInf")]
	pub mndt_rltd_inf: MandateRelatedInformation1,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: RemittanceInformation1,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: PartyIdentification8,
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification8,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: CashAccount7,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: CashAccount7,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: CashAccount7,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification8,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: CashAccount7,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: PartyIdentification8,
}


// AmountType2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AmountType2Choice {
	#[serde(rename = "InstdAmt")]
	pub instd_amt: CurrencyAndAmount,
	#[serde(rename = "EqvtAmt")]
	pub eqvt_amt: EquivalentAmount,
}


// EquivalentAmount 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EquivalentAmount {
	#[serde(rename = "Amt")]
	pub amt: CurrencyAndAmount,
	#[serde(rename = "CcyOfTrf")]
	pub ccy_of_trf: String,
}


// PartyIdentification8 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyIdentification8 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress1,
	#[serde(rename = "Id")]
	pub id: Party2Choice,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: String,
}


// PostalAddress1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: String,
	#[serde(rename = "AdrLine")]
	pub adr_line: Vec<String>,
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
}


// Party2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party2Choice {
	#[serde(rename = "OrgId")]
	pub org_id: OrganisationIdentification2,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Vec<PersonIdentification3>,
}


// OrganisationIdentification2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrganisationIdentification2 {
	#[serde(rename = "BIC")]
	pub bic: String,
	#[serde(rename = "IBEI")]
	pub ibei: String,
	#[serde(rename = "BEI")]
	pub bei: String,
	#[serde(rename = "EANGLN")]
	pub eangln: String,
	#[serde(rename = "USCHU")]
	pub uschu: String,
	#[serde(rename = "DUNS")]
	pub duns: String,
	#[serde(rename = "BkPtyId")]
	pub bk_pty_id: String,
	#[serde(rename = "TaxIdNb")]
	pub tax_id_nb: String,
	#[serde(rename = "PrtryId")]
	pub prtry_id: GenericIdentification3,
}


// BICIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BICIdentifier {
	#[serde(rename = "BICIdentifier")]
	pub bic_identifier: String,
}


// IBEIIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IBEIIdentifier {
	#[serde(rename = "IBEIIdentifier")]
	pub ibei_identifier: String,
}


// BEIIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BEIIdentifier {
	#[serde(rename = "BEIIdentifier")]
	pub bei_identifier: String,
}


// EANGLNIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EANGLNIdentifier {
	#[serde(rename = "EANGLNIdentifier")]
	pub eangln_identifier: String,
}


// CHIPSUniversalIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CHIPSUniversalIdentifier {
	#[serde(rename = "CHIPSUniversalIdentifier")]
	pub chips_universal_identifier: String,
}


// DunsIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DunsIdentifier {
	#[serde(rename = "DunsIdentifier")]
	pub duns_identifier: String,
}


// GenericIdentification3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// PersonIdentification3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonIdentification3 {
	#[serde(rename = "DrvrsLicNb")]
	pub drvrs_lic_nb: String,
	#[serde(rename = "CstmrNb")]
	pub cstmr_nb: String,
	#[serde(rename = "SclSctyNb")]
	pub scl_scty_nb: String,
	#[serde(rename = "AlnRegnNb")]
	pub aln_regn_nb: String,
	#[serde(rename = "PsptNb")]
	pub pspt_nb: String,
	#[serde(rename = "TaxIdNb")]
	pub tax_id_nb: String,
	#[serde(rename = "IdntyCardNb")]
	pub idnty_card_nb: String,
	#[serde(rename = "MplyrIdNb")]
	pub mplyr_id_nb: String,
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: DateAndPlaceOfBirth,
	#[serde(rename = "OthrId")]
	pub othr_id: GenericIdentification4,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// DateAndPlaceOfBirth 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateAndPlaceOfBirth {
	#[serde(rename = "BirthDt")]
	pub birth_dt: u8,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: String,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// GenericIdentification4 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericIdentification4 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IdTp")]
	pub id_tp: String,
}


// SettlementInformation3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementInformation3 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[serde(rename = "SttlmAcct")]
	pub sttlm_acct: CashAccount7,
	#[serde(rename = "ClrSys")]
	pub clr_sys: ClearingSystemIdentification1Choice,
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: CashAccount7,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: CashAccount7,
	#[serde(rename = "ThrdRmbrsmntAgt")]
	pub thrd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "ThrdRmbrsmntAgtAcct")]
	pub thrd_rmbrsmnt_agt_acct: CashAccount7,
}


// CashAccount7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccount7 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification3Choice,
	#[serde(rename = "Tp")]
	pub tp: CashAccountType2,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "Nm")]
	pub nm: String,
}


// AccountIdentification3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountIdentification3Choice {
	#[serde(rename = "IBAN")]
	pub iban: String,
	#[serde(rename = "BBAN")]
	pub bban: String,
	#[serde(rename = "UPIC")]
	pub upic: String,
	#[serde(rename = "PrtryAcct")]
	pub prtry_acct: SimpleIdentificationInformation2,
}


// IBANIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IBANIdentifier {
	#[serde(rename = "IBANIdentifier")]
	pub iban_identifier: String,
}


// BBANIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BBANIdentifier {
	#[serde(rename = "BBANIdentifier")]
	pub bban_identifier: String,
}


// UPICIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UPICIdentifier {
	#[serde(rename = "UPICIdentifier")]
	pub upic_identifier: String,
}


// SimpleIdentificationInformation2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleIdentificationInformation2 {
	#[serde(rename = "Id")]
	pub id: String,
}


// CashAccountType2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccountType2 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CashAccountType4Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccountType4Code {
	#[serde(rename = "CashAccountType4Code")]
	pub cash_account_type4_code: String,
}


// ClearingSystemIdentification1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification1Choice {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CashClearingSystem3Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashClearingSystem3Code {
	#[serde(rename = "CashClearingSystem3Code")]
	pub cash_clearing_system3_code: String,
}


// BranchAndFinancialInstitutionIdentification3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification3 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification5Choice,
	#[serde(rename = "BrnchId")]
	pub brnch_id: BranchData,
}


// FinancialInstitutionIdentification5Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionIdentification5Choice {
	#[serde(rename = "BIC")]
	pub bic: String,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: ClearingSystemMemberIdentification3Choice,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: NameAndAddress7,
	#[serde(rename = "PrtryId")]
	pub prtry_id: GenericIdentification3,
	#[serde(rename = "CmbndId")]
	pub cmbnd_id: FinancialInstitutionIdentification3,
}


// ClearingSystemMemberIdentification3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemMemberIdentification3Choice {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalClearingSystemMemberCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalClearingSystemMemberCode {
	#[serde(rename = "ExternalClearingSystemMemberCode")]
	pub external_clearing_system_member_code: String,
}


// NameAndAddress7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NameAndAddress7 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress1,
}


// FinancialInstitutionIdentification3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionIdentification3 {
	#[serde(rename = "BIC")]
	pub bic: String,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: ClearingSystemMemberIdentification3Choice,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress1,
	#[serde(rename = "PrtryId")]
	pub prtry_id: GenericIdentification3,
}


// BranchData 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchData {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress1,
}


// PaymentTypeInformation6 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentTypeInformation6 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: String,
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: ServiceLevel2Choice,
	#[serde(rename = "ClrChanl")]
	pub clr_chanl: String,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: LocalInstrument1Choice,
	#[serde(rename = "SeqTp")]
	pub seq_tp: String,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: String,
}


// ServiceLevel2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLevel2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ServiceLevel1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLevel1Code {
	#[serde(rename = "ServiceLevel1Code")]
	pub service_level1_code: String,
}


// LocalInstrument1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalInstrument1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ExternalLocalInstrumentCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalLocalInstrumentCode {
	#[serde(rename = "ExternalLocalInstrumentCode")]
	pub external_local_instrument_code: String,
}


// SequenceType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SequenceType1Code {
	#[serde(rename = "SequenceType1Code")]
	pub sequence_type1_code: String,
}


// PaymentCategoryPurpose1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentCategoryPurpose1Code {
	#[serde(rename = "PaymentCategoryPurpose1Code")]
	pub payment_category_purpose1_code: String,
}


// PaymentMethod4Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentMethod4Code {
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// MandateRelatedInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateRelatedInformation1 {
	#[serde(rename = "MndtId")]
	pub mndt_id: String,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: u8,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: bool,
	#[serde(rename = "AmdmntInfDtls")]
	pub amdmnt_inf_dtls: AmendmentInformationDetails1,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: String,
	#[serde(rename = "FrstColltnDt")]
	pub frst_colltn_dt: u8,
	#[serde(rename = "FnlColltnDt")]
	pub fnl_colltn_dt: u8,
	#[serde(rename = "Frqcy")]
	pub frqcy: String,
}


// TrueFalseIndicator 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// AmendmentInformationDetails1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AmendmentInformationDetails1 {
	#[serde(rename = "OrgnlMndtId")]
	pub orgnl_mndt_id: String,
	#[serde(rename = "OrgnlCdtrSchmeId")]
	pub orgnl_cdtr_schme_id: PartyIdentification8,
	#[serde(rename = "OrgnlCdtrAgt")]
	pub orgnl_cdtr_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "OrgnlCdtrAgtAcct")]
	pub orgnl_cdtr_agt_acct: CashAccount7,
	#[serde(rename = "OrgnlDbtr")]
	pub orgnl_dbtr: PartyIdentification8,
	#[serde(rename = "OrgnlDbtrAcct")]
	pub orgnl_dbtr_acct: CashAccount7,
	#[serde(rename = "OrgnlDbtrAgt")]
	pub orgnl_dbtr_agt: BranchAndFinancialInstitutionIdentification3,
	#[serde(rename = "OrgnlDbtrAgtAcct")]
	pub orgnl_dbtr_agt_acct: CashAccount7,
	#[serde(rename = "OrgnlFnlColltnDt")]
	pub orgnl_fnl_colltn_dt: u8,
	#[serde(rename = "OrgnlFrqcy")]
	pub orgnl_frqcy: String,
}


// Frequency1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frequency1Code {
	#[serde(rename = "Frequency1Code")]
	pub frequency1_code: String,
}


// Max1025Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// RemittanceInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceInformation1 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Vec<String>,
	#[serde(rename = "Strd")]
	pub strd: Vec<StructuredRemittanceInformation6>,
}


// StructuredRemittanceInformation6 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StructuredRemittanceInformation6 {
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: ReferredDocumentInformation1,
	#[serde(rename = "RfrdDocRltdDt")]
	pub rfrd_doc_rltd_dt: u8,
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: Vec<ReferredDocumentAmount1Choice>,
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: CreditorReferenceInformation1,
	#[serde(rename = "Invcr")]
	pub invcr: PartyIdentification8,
	#[serde(rename = "Invcee")]
	pub invcee: PartyIdentification8,
	#[serde(rename = "AddtlRmtInf")]
	pub addtl_rmt_inf: String,
}


// ReferredDocumentInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentInformation1 {
	#[serde(rename = "RfrdDocTp")]
	pub rfrd_doc_tp: ReferredDocumentType1,
	#[serde(rename = "RfrdDocNb")]
	pub rfrd_doc_nb: String,
}


// ReferredDocumentType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentType1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// DocumentType2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentType2Code {
	#[serde(rename = "DocumentType2Code")]
	pub document_type2_code: String,
}


// ReferredDocumentAmount1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentAmount1Choice {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: CurrencyAndAmount,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: CurrencyAndAmount,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: CurrencyAndAmount,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: CurrencyAndAmount,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: CurrencyAndAmount,
}


// CreditorReferenceInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorReferenceInformation1 {
	#[serde(rename = "CdtrRefTp")]
	pub cdtr_ref_tp: CreditorReferenceType1,
	#[serde(rename = "CdtrRef")]
	pub cdtr_ref: String,
}


// CreditorReferenceType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorReferenceType1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// DocumentType3Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentType3Code {
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}
