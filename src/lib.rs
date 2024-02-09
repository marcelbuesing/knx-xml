//!
//! A library for reading KNX project XML files.
//!
//! ```rust
//! use knx_xml::KNX;
//! use std::fs;
//! use yaserde::de::from_str;
//!
//! let content = fs::read_to_string("knx_master.xml").expect("Failed to read knx_master.xml");
//! let knx_hardware: KNX = from_str(&content).unwrap();
//! println!("{:#?}", knx_hardware);
//! ```
//!

use yaserde_derive::*;

// IDREF ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct IDREF {
    #[yaserde(rename = "IDREF")]
    pub idref: String,
}

// IDREFS ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct IDREFS {
    #[yaserde(rename = "IDREFS")]
    pub idrefs: Vec<String>,
}

// Capabilitiest ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Capabilitiest {
    #[yaserde(rename = "Capabilities_t")]
    pub capabilities_t: Vec<String>,
}

// String20t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct String20t {
    #[yaserde(rename = "String20_t")]
    pub string20_t: String,
}

// String50t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct String50t {
    #[yaserde(rename = "String50_t")]
    pub string50_t: String,
}

// String255t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct String255t {
    #[yaserde(rename = "String255_t")]
    pub string255_t: String,
}

// LanguageDependentStringt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LanguageDependentStringt {
    #[yaserde(rename = "LanguageDependentString_t")]
    pub language_dependent_string_t: String,
}

// LanguageDependentString20t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LanguageDependentString20t {
    #[yaserde(rename = "LanguageDependentString20_t")]
    pub language_dependent_string20_t: String,
}

// LanguageDependentString50t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LanguageDependentString50t {
    #[yaserde(rename = "LanguageDependentString50_t")]
    pub language_dependent_string50_t: String,
}

// LanguageDependentString255t ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LanguageDependentString255t {
    #[yaserde(rename = "LanguageDependentString255_t")]
    pub language_dependent_string255_t: String,
}

// Guidt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Guidt {
    #[yaserde(rename = "Guid_t")]
    pub guid_t: String,
}

// Ipv4Addresst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Ipv4Addresst {
    #[yaserde(rename = "Ipv4Address_t")]
    pub ipv4_address_t: String,
}

// RegistrationNumbert ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RegistrationNumbert {
    #[yaserde(rename = "RegistrationNumber_t")]
    pub registration_number_t: String,
}

// Regext ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Regext {
    #[yaserde(rename = "Regex_t")]
    pub regex_t: String,
}

// Valuet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Valuet {
    #[yaserde(rename = "Value_t")]
    pub value_t: String,
}

// Conditiont ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Conditiont {
    #[yaserde(rename = "Condition_t")]
    pub condition_t: String,
}

// BitOffsett ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BitOffsett {
    #[yaserde(rename = "BitOffset_t")]
    pub bit_offset_t: u16,
}

// HardwareVersionNumber ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct HardwareVersionNumber {
    #[yaserde(rename = "HardwareVersionNumber")]
    pub hardware_version_number: u16,
}

// AccessLevelt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AccessLevelt {
    #[yaserde(rename = "AccessLevel_t")]
    pub access_level_t: u16,
}

// FloatFormatt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FloatFormatt {
    #[yaserde(rename = "FloatFormat_t")]
    pub float_format_t: String,
}

// Aes128Keyt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Aes128Keyt {
    #[yaserde(rename = "Aes128Key_t")]
    pub aes128_key_t: String,
}

// PropTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct PropTypet {
    #[yaserde(rename = "PropType_t")]
    pub prop_type_t: String,
}

// LdCtrlMemAddrSpacet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlMemAddrSpacet {
    #[yaserde(rename = "LdCtrlMemAddrSpace_t")]
    pub ld_ctrl_mem_addr_space_t: String,
}

// LdCtrlControlVariablet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlControlVariablet {
    #[yaserde(rename = "LdCtrlControlVariable_t")]
    pub ld_ctrl_control_variable_t: String,
}

// LdCtrlProcTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlProcTypet {
    #[yaserde(rename = "LdCtrlProcType_t")]
    pub ld_ctrl_proc_type_t: String,
}

// LdCtrlErrorCauset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlErrorCauset {
    #[yaserde(rename = "LdCtrlErrorCause_t")]
    pub ld_ctrl_error_cause_t: String,
}

// MemoryTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MemoryTypet {
    #[yaserde(rename = "MemoryType_t")]
    pub memory_type_t: String,
}

// Accesst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Accesst {
    #[yaserde(rename = "Access_t")]
    pub access_t: String,
}

// ComObjectPriorityt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectPriorityt {
    #[yaserde(rename = "ComObjectPriority_t")]
    pub com_object_priority_t: String,
}

// ComObjectSizet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectSizet {
    #[yaserde(rename = "ComObjectSize_t")]
    pub com_object_size_t: String,
}

// Enablet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Enablet {
    #[yaserde(rename = "Enable_t")]
    pub enable_t: String,
}

// LoadProcedureStylet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LoadProcedureStylet {
    #[yaserde(rename = "LoadProcedureStyle_t")]
    pub load_procedure_style_t: String,
}

// CompletionStatust ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct CompletionStatust {
    #[yaserde(rename = "CompletionStatus_t")]
    pub completion_status_t: String,
}

// ResourceNamet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceNamet {
    #[yaserde(rename = "ResourceName_t")]
    pub resource_name_t: String,
}

// ResourceAccesst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceAccesst {
    #[yaserde(rename = "ResourceAccess_t")]
    pub resource_access_t: String,
}

// ResourceMgmtStylet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceMgmtStylet {
    #[yaserde(rename = "ResourceMgmtStyle_t")]
    pub resource_mgmt_style_t: String,
}

// ResourceAddrSpacet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceAddrSpacet {
    #[yaserde(rename = "ResourceAddrSpace_t")]
    pub resource_addr_space_t: String,
}

// ResourceAccessRightst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceAccessRightst {
    #[yaserde(rename = "ResourceAccessRights_t")]
    pub resource_access_rights_t: String,
}

// ProcedureTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProcedureTypet {
    #[yaserde(rename = "ProcedureType_t")]
    pub procedure_type_t: String,
}

// GroupAddressStylet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupAddressStylet {
    #[yaserde(rename = "GroupAddressStyle_t")]
    pub group_address_style_t: String,
}

// SpaceTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SpaceTypet {
    #[yaserde(rename = "SpaceType_t")]
    pub space_type_t: String,
}

// ApplicationProgramTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramTypet {
    #[yaserde(rename = "ApplicationProgramType_t")]
    pub application_program_type_t: String,
}

// RegistrationStatust ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RegistrationStatust {
    #[yaserde(rename = "RegistrationStatus_t")]
    pub registration_status_t: String,
}

// ProjectTracingLevelt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProjectTracingLevelt {
    #[yaserde(rename = "ProjectTracingLevel_t")]
    pub project_tracing_level_t: String,
}

// ToDoStatust ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ToDoStatust {
    #[yaserde(rename = "ToDoStatus_t")]
    pub to_do_status_t: String,
}

// Capabilityt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Capabilityt {
    #[yaserde(rename = "Capability_t")]
    pub capability_t: String,
}

// ApplicationProgramIPConfigt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramIPConfigt {
    #[yaserde(rename = "ApplicationProgramIPConfig_t")]
    pub application_program_ip_config_t: String,
}

// IPConfigAssignt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct IPConfigAssignt {
    #[yaserde(rename = "IPConfigAssign_t")]
    pub ip_config_assign_t: String,
}

// ComTableExpectationt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComTableExpectationt {
    #[yaserde(rename = "ComTableExpectation_t")]
    pub com_table_expectation_t: String,
}

// HorizontalAlignmentt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct HorizontalAlignmentt {
    #[yaserde(rename = "HorizontalAlignment_t")]
    pub horizontal_alignment_t: String,
}

// RFDeviceModet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RFDeviceModet {
    #[yaserde(rename = "RFDeviceMode_t")]
    pub rf_device_mode_t: String,
}

// TextEncodingt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TextEncodingt {
    #[yaserde(rename = "TextEncoding_t")]
    pub text_encoding_t: String,
}

// DownloadBehaviort ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DownloadBehaviort {
    #[yaserde(rename = "DownloadBehavior_t")]
    pub download_behavior_t: String,
}

// SecurityModet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SecurityModet {
    #[yaserde(rename = "SecurityMode_t")]
    pub security_mode_t: String,
}

// ComObjectSecurityRequirementst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectSecurityRequirementst {
    #[yaserde(rename = "ComObjectSecurityRequirements_t")]
    pub com_object_security_requirements_t: String,
}

// CellReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct CellReft {
    #[yaserde(rename = "CellRef_t")]
    pub cell_ref_t: String,
}

// ParameterBlockLayoutt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterBlockLayoutt {
    #[yaserde(rename = "ParameterBlockLayout_t")]
    pub parameter_block_layout_t: String,
}

// DeprecationStatust ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeprecationStatust {
    #[yaserde(rename = "DeprecationStatus_t")]
    pub deprecation_status_t: String,
}

// KNX ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct KNX {
    #[yaserde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    #[yaserde(rename = "ToolVersion")]
    pub tool_version: Option<String>,
    #[yaserde(rename = "MasterData")]
    pub master_data: Option<MasterDatat>,
    #[yaserde(rename = "ManufacturerData")]
    pub manufacturer_data: Option<ManufacturerDatat>,
    #[yaserde(rename = "Project")]
    pub project: Vec<Projectt>,
}

// DatapointTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointTypes {
    #[yaserde(rename = "DatapointType")]
    pub datapoint_type: Vec<DatapointTypet>,
}

// DatapointRoles ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointRoles {
    #[yaserde(rename = "DatapointRole")]
    pub datapoint_role: Vec<DatapointRolet>,
}

// InterfaceObjectType ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObjectType {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
}

// InterfaceObjectTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObjectTypes {
    #[yaserde(rename = "InterfaceObjectType")]
    pub interface_object_type: Vec<InterfaceObjectType>,
}

// InterfaceObjectProperty ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObjectProperty {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(rename = "ObjectType")]
    pub object_type: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "PDT")]
    pub pdt: Vec<String>,
    #[yaserde(rename = "DPT")]
    pub dpt: Option<String>,
    #[yaserde(rename = "Array")]
    pub array: Option<bool>,
}

// InterfaceObjectProperties ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObjectProperties {
    #[yaserde(rename = "InterfaceObjectProperty")]
    pub interface_object_property: Vec<InterfaceObjectProperty>,
}

// PropertyDataType ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct PropertyDataType {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Size")]
    pub size: Option<u32>,
    #[yaserde(rename = "ReadSize")]
    pub read_size: Option<u16>,
}

// PropertyDataTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct PropertyDataTypes {
    #[yaserde(rename = "PropertyDataType")]
    pub property_data_type: Vec<PropertyDataType>,
}

// MediumType ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MediumType {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(attribute, rename = "DomainAddressLength")]
    pub domain_address_length: i16,
}

// MediumTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MediumTypes {
    #[yaserde(rename = "MediumType")]
    pub medium_type: Vec<MediumType>,
}

// MaskVersions ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MaskVersions {
    #[yaserde(rename = "MaskVersion")]
    pub mask_version: Vec<MaskVersiont>,
}

// Parameter ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Parameter {
    #[yaserde(attribute, rename = "Property")]
    pub property: Option<String>,
    #[yaserde(attribute, rename = "Description")]
    pub description: Option<String>,
}

// Parameters ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Parameters {
    #[yaserde(attribute, rename = "ObjectType")]
    pub object_type: Option<String>,
    #[yaserde(rename = "Parameter")]
    pub parameter: Vec<Parameter>,
}

// FunctionalBlock ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionalBlock {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "Parameters")]
    pub parameters: Vec<Parameters>,
}

// FunctionalBlocks ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionalBlocks {
    #[yaserde(rename = "FunctionalBlock")]
    pub functional_block: Vec<FunctionalBlock>,
}

// Language ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Language {
    #[yaserde(rename = "Identifier")]
    pub identifier: Option<String>,
}

// ProductLanguages ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProductLanguages {
    #[yaserde(rename = "Language")]
    pub language: Vec<Language>,
}

// FunctionTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionTypes {
    #[yaserde(rename = "FunctionsGroup")]
    pub functions_group: Vec<FunctionsGroupt>,
    #[yaserde(rename = "FunctionType")]
    pub function_type: Vec<FunctionTypet>,
}

// SpaceUsages ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SpaceUsages {
    #[yaserde(rename = "SpaceUsage")]
    pub space_usage: Vec<SpaceUsaget>,
}

// RSAKeyValue ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RSAKeyValue {
    #[yaserde(rename = "Modulus")]
    pub modulus: String,
    #[yaserde(rename = "Exponent")]
    pub exponent: String,
}

// PublicKey ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct PublicKey {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(rename = "Revoked")]
    pub revoked: Option<bool>,
    #[yaserde(rename = "RSAKeyValue")]
    pub rsa_key_value: RSAKeyValue,
}

// PublicKeys ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct PublicKeys {
    #[yaserde(rename = "PublicKey")]
    pub public_key: Vec<PublicKey>,
}

// Manufacturer ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Manufacturer {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "KnxManufacturerId")]
    pub knx_manufacturer_id: Option<u16>,
    #[yaserde(attribute, rename = "DefaultLanguage")]
    pub default_language: Option<String>,
    #[yaserde(attribute, rename = "CompatibilityGroup")]
    pub compatibility_group: Option<u16>,
    #[yaserde(attribute, rename = "ImportRestriction")]
    pub import_restriction: Option<String>,
    #[yaserde(attribute, rename = "ImportGroup")]
    pub import_group: Option<String>,
    #[yaserde(rename = "OrderNumberFormattingScript")]
    pub order_number_formatting_script: Option<String>,
    #[yaserde(rename = "PublicKeys")]
    pub public_keys: Option<PublicKeys>,
    #[yaserde(rename = "DatapointTypes")]
    pub datapoint_types: Option<DatapointTypes>,
    #[yaserde(rename = "DatapointRoles")]
    pub datapoint_roles: Option<DatapointRoles>,
    #[yaserde(rename = "FunctionTypes")]
    pub function_types: Option<FunctionTypes>,
    #[yaserde(rename = "SpaceUsages")]
    pub space_usages: Option<SpaceUsages>,
    #[yaserde(rename = "ApplicationPrograms")]
    pub application_programs: Option<ApplicationPrograms>,
}

// Manufacturers ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Manufacturers {
    #[yaserde(rename = "Manufacturer")]
    pub manufacturer: Vec<Manufacturer>,
}

// Languages ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Languages {
    #[yaserde(rename = "Language")]
    pub language: Vec<LanguageDatat>,
}

// MasterDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MasterDatat {
    #[yaserde(attribute, rename = "Version")]
    pub version: u32,
    #[yaserde(attribute, rename = "Signature")]
    pub signature: String,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "DatapointTypes")]
    pub datapoint_types: DatapointTypes,
    #[yaserde(rename = "DatapointRoles")]
    pub datapoint_roles: DatapointRoles,
    #[yaserde(rename = "InterfaceObjectTypes")]
    pub interface_object_types: InterfaceObjectTypes,
    #[yaserde(rename = "InterfaceObjectProperties")]
    pub interface_object_properties: InterfaceObjectProperties,
    #[yaserde(rename = "PropertyDataTypes")]
    pub property_data_types: PropertyDataTypes,
    #[yaserde(rename = "MediumTypes")]
    pub medium_types: MediumTypes,
    #[yaserde(rename = "MaskVersions")]
    pub mask_versions: MaskVersions,
    #[yaserde(rename = "FunctionalBlocks")]
    pub functional_blocks: FunctionalBlocks,
    #[yaserde(rename = "ProductLanguages")]
    pub product_languages: ProductLanguages,
    #[yaserde(rename = "FunctionTypes")]
    pub function_types: FunctionTypes,
    #[yaserde(rename = "SpaceUsages")]
    pub space_usages: SpaceUsages,
    #[yaserde(rename = "Manufacturers")]
    pub manufacturers: Manufacturers,
    #[yaserde(rename = "Languages")]
    pub languages: Languages,
}

// Bit ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Bit {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Set")]
    pub set: String,
    #[yaserde(attribute, rename = "Cleared")]
    pub cleared: Option<String>,
}

// UnsignedInteger ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct UnsignedInteger {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Width")]
    pub width: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Unit")]
    pub unit: Option<String>,
    #[yaserde(rename = "MinInclusive")]
    pub min_inclusive: Option<u64>,
    #[yaserde(rename = "MaxInclusive")]
    pub max_inclusive: Option<u64>,
    #[yaserde(rename = "Coefficient")]
    pub coefficient: Option<f64>,
}

// SignedInteger ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SignedInteger {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Width")]
    pub width: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Unit")]
    pub unit: Option<String>,
    #[yaserde(rename = "MinInclusive")]
    pub min_inclusive: Option<i64>,
    #[yaserde(rename = "MaxInclusive")]
    pub max_inclusive: Option<i64>,
    #[yaserde(rename = "Coefficient")]
    pub coefficient: Option<f64>,
}

// String ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct StringT {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Width")]
    pub width: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Unit")]
    pub unit: Option<String>,
    #[yaserde(rename = "Encoding")]
    pub encoding: Option<String>,
}

// Float ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Float {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Width")]
    pub width: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Unit")]
    pub unit: Option<String>,
    #[yaserde(rename = "Coefficient")]
    pub coefficient: Option<f64>,
    #[yaserde(rename = "MinValue")]
    pub min_value: Option<f64>,
    #[yaserde(rename = "MaxValue")]
    pub max_value: Option<f64>,
}

// EnumValue ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct EnumValue {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Value")]
    pub value: i32,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
}

// Enumeration ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Enumeration {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Width")]
    pub width: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "EnumValue")]
    pub enum_value: Vec<EnumValue>,
}

// Reserved ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Reserved {
    #[yaserde(attribute, rename = "Width")]
    pub width: i32,
}

// RefType ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RefType {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
}

// Format ...
#[derive(Debug, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub enum Format {
    #[yaserde(rename = "Bit")]
    Bit(Bit),
    #[yaserde(rename = "UnsignedInteger")]
    UnsignedInteger(UnsignedInteger),
    #[yaserde(rename = "SignedInteger")]
    SignedInteger(SignedInteger),
    #[yaserde(rename = "String")]
    String(StringT),
    #[yaserde(rename = "Float")]
    Float(Float),
    #[yaserde(rename = "Enumeration")]
    Enumeration(Enumeration),
    #[yaserde(rename = "Reserved")]
    Reserved(Reserved),
    #[yaserde(rename = "RefType")]
    RefType(RefType),
}

impl Default for Format {
    fn default() -> Format {
        Format::Bit(Default::default())
    }
}

// DatapointSubtype ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointSubtype {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(attribute, rename = "Default")]
    pub default: Option<bool>,
    #[yaserde(attribute, rename = "PDT")]
    pub pdt: Option<String>,
    #[yaserde(rename = "Format")]
    pub format: Option<Format>,
}

// DatapointSubtypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointSubtypes {
    #[yaserde(rename = "DatapointSubtype")]
    pub datapoint_subtype: Vec<DatapointSubtype>,
}

// DatapointTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointTypet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(attribute, rename = "SizeInBit")]
    pub size_in_bit: u32,
    #[yaserde(attribute, rename = "Default")]
    pub default: Option<bool>,
    #[yaserde(attribute, rename = "PDT")]
    pub pdt: Option<String>,
    #[yaserde(rename = "DatapointSubtypes")]
    pub datapoint_subtypes: DatapointSubtypes,
}

// DatapointRolet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DatapointRolet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Status")]
    pub status: Option<String>,
}

// DownwardCompatibleMask ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DownwardCompatibleMask {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
}

// DownwardCompatibleMasks ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DownwardCompatibleMasks {
    #[yaserde(rename = "DownwardCompatibleMask")]
    pub downward_compatible_mask: Vec<DownwardCompatibleMask>,
}

// MaskEntry ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MaskEntry {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Address")]
    pub address: u16,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
}

// MaskEntries ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MaskEntries {
    #[yaserde(rename = "MaskEntry")]
    pub mask_entry: Vec<MaskEntry>,
}

// MaskVersiont ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MaskVersiont {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "MaskVersion")]
    pub mask_version: u16,
    #[yaserde(attribute, rename = "MgmtDescriptor01")]
    pub mgmt_descriptor01: Option<String>,
    #[yaserde(attribute, rename = "ManagementModel")]
    pub management_model: String,
    #[yaserde(attribute, rename = "MediumTypeRefId")]
    pub medium_type_ref_id: String,
    #[yaserde(attribute, rename = "OtherMediumTypeRefId")]
    pub other_medium_type_ref_id: Option<String>,
    #[yaserde(rename = "DownwardCompatibleMasks")]
    pub downward_compatible_masks: Option<DownwardCompatibleMasks>,
    #[yaserde(rename = "MaskEntries")]
    pub mask_entries: Option<MaskEntries>,
    #[yaserde(rename = "HawkConfigurationData")]
    pub hawk_configuration_data: Vec<HawkConfigurationDatat>,
}

// ResourceLocationt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceLocationt {
    #[yaserde(attribute, rename = "AddressSpace")]
    pub address_space: String,
    #[yaserde(attribute, rename = "InterfaceObjectRef")]
    pub interface_object_ref: Option<u16>,
    #[yaserde(attribute, rename = "PropertyID")]
    pub property_id: Option<u16>,
    #[yaserde(attribute, rename = "StartAddress")]
    pub start_address: Option<u32>,
    #[yaserde(attribute, rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(attribute, rename = "PtrResource")]
    pub ptr_resource: Option<String>,
}

// Feature ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Feature {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Value")]
    pub value: i32,
}

// Features ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Features {
    #[yaserde(rename = "Feature")]
    pub feature: Vec<Feature>,
}

// ResourceType ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ResourceType {
    #[yaserde(attribute, rename = "Length")]
    pub length: u32,
    #[yaserde(attribute, rename = "Flavour")]
    pub flavour: Option<String>,
}

// AccessRights ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AccessRights {
    #[yaserde(attribute, rename = "Read")]
    pub read: String,
    #[yaserde(attribute, rename = "Write")]
    pub write: String,
}

// Resource ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Resource {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Access")]
    pub access: String,
    #[yaserde(attribute, rename = "MgmtStyle")]
    pub mgmt_style: Option<String>,
    #[yaserde(rename = "Location")]
    pub location: Option<ResourceLocationt>,
    #[yaserde(rename = "ImgLocation")]
    pub img_location: Option<ResourceLocationt>,
    #[yaserde(rename = "ResourceType")]
    pub resource_type: ResourceType,
    #[yaserde(rename = "AccessRights")]
    pub access_rights: AccessRights,
}

// Resources ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Resources {
    #[yaserde(rename = "Resource")]
    pub resource: Vec<Resource>,
}

// Procedure ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Procedure {
    #[yaserde(attribute, rename = "ProcedureType")]
    pub procedure_type: String,
    #[yaserde(attribute, rename = "ProcedureSubType")]
    pub procedure_sub_type: String,
    #[yaserde(attribute, rename = "Access")]
    pub access: String,
    #[yaserde(flatten)]
    pub load_proceduret: LoadProceduret,
}

// Procedures ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Procedures {
    #[yaserde(rename = "Procedure")]
    pub procedure: Vec<Procedure>,
}

// MemorySegment ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MemorySegment {
    #[yaserde(attribute, rename = "Length")]
    pub length: u32,
    #[yaserde(attribute, rename = "Optional")]
    pub optional: Option<bool>,
    #[yaserde(attribute, rename = "MemoryType")]
    pub memory_type: Option<String>,
    #[yaserde(rename = "Location")]
    pub location: ResourceLocationt,
    #[yaserde(rename = "AccessRights")]
    pub access_rights: AccessRights,
}

// MemorySegments ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct MemorySegments {
    #[yaserde(rename = "MemorySegment")]
    pub memory_segment: Vec<MemorySegment>,
}

// Property ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Property {
    #[yaserde(attribute, rename = "PropertyID")]
    pub property_id: u16,
    #[yaserde(attribute, rename = "PropertyDataType")]
    pub property_data_type: Option<String>,
}

// InterfaceObject ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObject {
    #[yaserde(attribute, rename = "Index")]
    pub index: Option<u16>,
    #[yaserde(attribute, rename = "ObjectType")]
    pub object_type: u16,
    #[yaserde(rename = "Property")]
    pub property: Vec<Property>,
}

// InterfaceObjects ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct InterfaceObjects {
    #[yaserde(rename = "InterfaceObject")]
    pub interface_object: Vec<InterfaceObject>,
}

// HawkConfigurationDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct HawkConfigurationDatat {
    #[yaserde(rename = "Ets3SystemPlugin")]
    pub ets3_system_plugin: Option<String>,
    #[yaserde(rename = "LegacyVersion")]
    pub legacy_version: Option<i32>,
    #[yaserde(rename = "Features")]
    pub features: Vec<Features>,
    #[yaserde(rename = "Resources")]
    pub resources: Vec<Resources>,
    #[yaserde(rename = "Procedures")]
    pub procedures: Vec<Procedures>,
    #[yaserde(rename = "MemorySegments")]
    pub memory_segments: Vec<MemorySegments>,
    #[yaserde(rename = "InterfaceObjects")]
    pub interface_objects: Vec<InterfaceObjects>,
}

// SpaceUsaget ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SpaceUsaget {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Relations")]
    pub relations: Vec<String>,
    #[yaserde(rename = "Status")]
    pub status: Option<String>,
    #[yaserde(rename = "SpaceUsage")]
    pub space_usage: Vec<SpaceUsaget>,
}

// FunctionPoint ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionPoint {
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(attribute, rename = "Role")]
    pub role: String,
    #[yaserde(attribute, rename = "DatapointType")]
    pub datapoint_type: String,
    #[yaserde(attribute, rename = "Characteristics")]
    pub characteristics: Option<String>,
}

// FunctionTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionTypet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Status")]
    pub status: Option<String>,
    #[yaserde(rename = "FunctionPoint")]
    pub function_point: Vec<FunctionPoint>,
}

// FunctionsGroupt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FunctionsGroupt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Status")]
    pub status: Option<String>,
    #[yaserde(rename = "FunctionsGroup")]
    pub functions_group: Vec<FunctionsGroupt>,
    #[yaserde(rename = "FunctionType")]
    pub function_type: Vec<FunctionTypet>,
}

// Catalog ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Catalog {
    #[yaserde(rename = "CatalogSection")]
    pub catalog_section: Vec<CatalogSectiont>,
}

// ApplicationPrograms ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationPrograms {
    #[yaserde(rename = "ApplicationProgram")]
    pub application_program: Vec<ApplicationProgramt>,
}

// FileInfo ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FileInfo {
    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
    #[yaserde(attribute, rename = "TimeInfo")]
    pub time_info: Option<u16>,
    #[yaserde(attribute, rename = "Hidden")]
    pub hidden: Option<bool>,
    #[yaserde(attribute, rename = "ReadOnly")]
    pub read_only: Option<bool>,
}

// Baggage ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Baggage {
    #[yaserde(attribute, rename = "TargetPath")]
    pub target_path: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "FileIntegrity")]
    pub file_integrity: Option<String>,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "FileInfo")]
    pub file_info: FileInfo,
}

// Baggages ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Baggages {
    #[yaserde(rename = "Baggage")]
    pub baggage: Vec<Baggage>,
}

// Hardware ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Hardware {
    #[yaserde(rename = "Hardware")]
    pub hardware: Vec<Hardwaret>,
}

// ManufacturerDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ManufacturerDatat {
    #[yaserde(rename = "Manufacturer")]
    pub manufacturer: Vec<Manufacturer>,
}

// ApplicationProgramt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "ApplicationNumber")]
    pub application_number: u16,
    #[yaserde(attribute, rename = "ApplicationVersion")]
    pub application_version: u16,
    #[yaserde(attribute, rename = "ProgramType")]
    pub program_type: String,
    #[yaserde(attribute, rename = "MaskVersion")]
    pub mask_version: String,
    #[yaserde(attribute, rename = "VisibleDescription")]
    pub visible_description: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "LoadProcedureStyle")]
    pub load_procedure_style: String,
    #[yaserde(attribute, rename = "PeiType")]
    pub pei_type: u16,
    #[yaserde(attribute, rename = "HelpTopic")]
    pub help_topic: Option<u32>,
    #[yaserde(attribute, rename = "HelpFile")]
    pub help_file: Option<String>,
    #[yaserde(attribute, rename = "ContextHelpFile")]
    pub context_help_file: Option<String>,
    #[yaserde(attribute, rename = "IconFile")]
    pub icon_file: Option<String>,
    #[yaserde(attribute, rename = "DefaultLanguage")]
    pub default_language: String,
    #[yaserde(attribute, rename = "DynamicTableManagement")]
    pub dynamic_table_management: bool,
    #[yaserde(attribute, rename = "Linkable")]
    pub linkable: bool,
    #[yaserde(attribute, ename = "IsSecureEnabled")]
    pub is_secure_enabled: Option<bool>,
    #[yaserde(attribute, rename = "MinEtsVersion")]
    pub min_ets_version: Option<String>,
    #[yaserde(attribute, rename = "OriginalManufacturer")]
    pub original_manufacturer: Option<String>,
    #[yaserde(attribute, rename = "PreEts4Style")]
    pub pre_ets4_style: Option<bool>,
    #[yaserde(attribute, rename = "ConvertedFromPreEts4Data")]
    pub converted_from_pre_ets4_data: Option<bool>,
    #[yaserde(attribute, rename = "CreatedFromLegacySchemaVersion")]
    pub created_from_legacy_schema_version: Option<bool>,
    #[yaserde(attribute, rename = "IPConfig")]
    pub ip_config: Option<String>,
    #[yaserde(attribute, rename = "AdditionalAddressesCount")]
    pub additional_addresses_count: Option<i32>,
    #[yaserde(attribute, rename = "MaxUserEntries")]
    pub max_user_entries: Option<u16>,
    #[yaserde(attribute, rename = "MaxTunnelingUserEntries")]
    pub max_tunneling_user_entries: Option<u16>,
    #[yaserde(attribute, rename = "MaxSecurityIndividualAddressEntries")]
    pub max_security_individual_address_entries: Option<u16>,
    #[yaserde(attribute, rename = "MaxSecurityGroupKeyTableEntries")]
    pub max_security_group_key_table_entries: Option<u16>,
    #[yaserde(attribute, rename = "MaxSecurityP2PKeyTableEntries")]
    pub max_security_p2_p_key_table_entries: Option<u16>,
    #[yaserde(attribute, rename = "NonRegRelevantDataVersion")]
    pub non_reg_relevant_data_version: Option<u16>,
    #[yaserde(attribute, rename = "Broken")]
    pub broken: Option<bool>,
    #[yaserde(attribute, rename = "DownloadInfoIncomplete")]
    pub download_info_incomplete: Option<bool>,
    #[yaserde(attribute, rename = "ReplacesVersions")]
    pub replaces_versions: Option<u16>,
    #[yaserde(attribute, rename = "Hash")]
    pub hash: Option<String>,
    #[yaserde(attribute, rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Static")]
    pub static_attr: ApplicationProgramStatict,
    #[yaserde(rename = "Dynamic")]
    pub dynamic: ApplicationProgramDynamict,
}

// AbsoluteSegment is registration-relevant
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AbsoluteSegment {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "MemoryType")]
    pub memory_type: Option<String>,
    #[yaserde(attribute, rename = "Address")]
    pub address: u32,
    #[yaserde(attribute, rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "UserMemory")]
    pub user_memory: Option<bool>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Data")]
    pub data: Option<String>,
    #[yaserde(rename = "Mask")]
    pub mask: Option<String>,
}

// RelativeSegment ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RelativeSegment {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "LoadStateMachine")]
    pub load_state_machine: u16,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Data")]
    pub data: String,
    #[yaserde(rename = "Mask")]
    pub mask: String,
}

// Code ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Code {
    #[yaserde(rename = "AbsoluteSegment")]
    pub absolute_segment: Vec<AbsoluteSegment>,
    #[yaserde(rename = "RelativeSegment")]
    pub relative_segment: Vec<RelativeSegment>,
}

// ParameterTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterTypes {
    #[yaserde(rename = "ParameterType")]
    pub parameter_type: Vec<ParameterTypet>,
}

// Memory ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Memory {
    #[yaserde(rename = "CodeSegment")]
    pub code_segment: String,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "BitOffset")]
    pub bit_offset: u16,
}

// Union ...
#[derive(Debug, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub enum Union {
    #[yaserde(rename = "SizeInBit")]
    SizeInBit(u32),
    #[yaserde(rename = "Memory")]
    Memory(Memory),
    #[yaserde(rename = "Property")]
    Property(Property),
    #[yaserde(rename = "Parameter")]
    Parameter(UnionParametert),
}

impl Default for Union {
    fn default() -> Union {
        Union::SizeInBit(Default::default())
    }
}

// ParameterRefs ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterRefs {
    #[yaserde(rename = "ParameterRef")]
    pub parameter_ref: Vec<ParameterReft>,
}

// ParameterCalculations ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterCalculations {
    #[yaserde(rename = "ParameterCalculation")]
    pub parameter_calculation: Vec<ParameterCalculationt>,
}

// ParameterValidations ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterValidations {
    #[yaserde(rename = "ParameterValidation")]
    pub parameter_validation: Vec<ParameterValidationt>,
}

// ComObjectTable ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectTable {
    #[yaserde(rename = "CodeSegment")]
    pub code_segment: Option<String>,
    #[yaserde(rename = "Offset")]
    pub offset: Option<u32>,
    #[yaserde(rename = "ComObject")]
    pub com_object: Vec<ComObjectt>,
}

// ComObjectRefs ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectRefs {
    #[yaserde(rename = "ComObjectRef")]
    pub com_object_ref: Vec<ComObjectReft>,
}

// AddressTable ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AddressTable {
    #[yaserde(attribute, rename = "CodeSegment")]
    pub code_segment: Option<String>,
    #[yaserde(attribute, rename = "Offset")]
    pub offset: Option<u32>,
    #[yaserde(attribute, rename = "MaxEntries")]
    pub max_entries: u32,
}

// AssociationTable ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AssociationTable {
    #[yaserde(attribute, rename = "CodeSegment")]
    pub code_segment: Option<String>,
    #[yaserde(attribute, rename = "Offset")]
    pub offset: Option<u32>,
    #[yaserde(attribute, rename = "MaxEntries")]
    pub max_entries: u32,
}

// FixupList ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct FixupList {
    #[yaserde(rename = "Fixup")]
    pub fixup: Vec<Fixupt>,
}

// Extension ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Extension {
    #[yaserde(attribute, rename = "EtsDownloadPlugin")]
    pub ets_download_plugin: Option<String>,
    #[yaserde(attribute, rename = "EtsUiPlugin")]
    pub ets_ui_plugin: Option<String>,
    #[yaserde(attribute, rename = "EtsDataHandler")]
    pub ets_data_handler: Option<String>,
    #[yaserde(attribute, rename = "EtsDataHandlerCapabilities")]
    pub ets_data_handler_capabilities: Option<Capabilitiest>,
    #[yaserde(attribute, rename = "RequiresExternalSoftware")]
    pub requires_external_software: Option<bool>,
    #[yaserde(rename = "Baggage")]
    pub baggage: Vec<BaggageExtension>,
}

#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BaggageExtension {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
}

// BinaryData ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BinaryData {
    #[yaserde(rename = "BinaryData")]
    pub binary_data: Vec<BinaryDatat>,
}

// ExcludeMemory ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ExcludeMemory {
    #[yaserde(rename = "CodeSegment")]
    pub code_segment: String,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// ExcludeProperty ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ExcludeProperty {
    #[yaserde(rename = "ObjectIndex")]
    pub object_index: Option<u16>,
    #[yaserde(rename = "ObjectType")]
    pub object_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropertyId")]
    pub property_id: u16,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// DeviceCompare ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeviceCompare {
    #[yaserde(rename = "StandardComTablesExpectable")]
    pub standard_com_tables_expectable: Option<String>,
    #[yaserde(rename = "ExcludeMemory")]
    pub exclude_memory: Vec<ExcludeMemory>,
    #[yaserde(rename = "ExcludeProperty")]
    pub exclude_property: Vec<ExcludeProperty>,
}

// Message ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Message {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
}

// Messages ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Messages {
    #[yaserde(rename = "Message")]
    pub message: Vec<Message>,
}

// Script ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Script {
    #[yaserde(rename = "$value")]
    pub value: String,
}

// SecurityRole ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SecurityRole {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Mask")]
    pub mask: u16,
}

// SecurityRoles ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SecurityRoles {
    #[yaserde(rename = "SecurityRole")]
    pub security_role: Vec<SecurityRole>,
}

// BusInterface ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BusInterface {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "AddressIndex")]
    pub address_index: u16,
    #[yaserde(rename = "AccessType")]
    pub access_type: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
}

// BusInterfaces ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BusInterfaces {
    #[yaserde(rename = "BusInterface")]
    pub bus_interface: Vec<BusInterface>,
}

// Options ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Options {
    #[yaserde(rename = "PreferPartialDownloadIfApplicationLoaded")]
    pub prefer_partial_download_if_application_loaded: Option<bool>,
    #[yaserde(rename = "EasyCtrlModeModeStyleEmptyGroupComTables")]
    pub easy_ctrl_mode_mode_style_empty_group_com_tables: Option<bool>,
    #[yaserde(rename = "SetObjectTableLengthAlwaysToOne")]
    pub set_object_table_length_always_to_one: Option<bool>,
    #[yaserde(rename = "TextParameterEncoding")]
    pub text_parameter_encoding: Option<String>,
    #[yaserde(rename = "TextParameterEncodingSelector")]
    pub text_parameter_encoding_selector: Option<String>,
    #[yaserde(rename = "TextParameterZeroTerminate")]
    pub text_parameter_zero_terminate: Option<bool>,
    #[yaserde(rename = "ParameterByteOrder")]
    pub parameter_byte_order: Option<String>,
    #[yaserde(rename = "PartialDownloadOnlyVisibleParameters")]
    pub partial_download_only_visible_parameters: Option<bool>,
    #[yaserde(rename = "LegacyNoPartialDownload")]
    pub legacy_no_partial_download: Option<bool>,
    #[yaserde(rename = "LegacyNoMemoryVerifyMode")]
    pub legacy_no_memory_verify_mode: Option<bool>,
    #[yaserde(rename = "LegacyNoOptimisticWrite")]
    pub legacy_no_optimistic_write: Option<bool>,
    #[yaserde(rename = "LegacyDoNotReportPropertyWriteErrors")]
    pub legacy_do_not_report_property_write_errors: Option<bool>,
    #[yaserde(rename = "LegacyNoBackgroundDownload")]
    pub legacy_no_background_download: Option<bool>,
    #[yaserde(rename = "LegacyDoNotCheckManufacturerId")]
    pub legacy_do_not_check_manufacturer_id: Option<bool>,
    #[yaserde(rename = "LegacyAlwaysReloadAppIfCoVisibilityChanged")]
    pub legacy_always_reload_app_if_co_visibility_changed: Option<bool>,
    #[yaserde(rename = "LegacyNeverReloadAppIfCoVisibilityChanged")]
    pub legacy_never_reload_app_if_co_visibility_changed: Option<bool>,
    #[yaserde(rename = "LegacyDoNotSupportUndoDelete")]
    pub legacy_do_not_support_undo_delete: Option<bool>,
    #[yaserde(rename = "LegacyAllowPartialDownloadIfAp2Mismatch")]
    pub legacy_allow_partial_download_if_ap2_mismatch: Option<bool>,
    #[yaserde(rename = "LegacyKeepObjectTableGaps")]
    pub legacy_keep_object_table_gaps: Option<bool>,
    #[yaserde(rename = "LegacyProxyCommunicationObjects")]
    pub legacy_proxy_communication_objects: Option<bool>,
    #[yaserde(rename = "DeviceInfoIgnoreRunState")]
    pub device_info_ignore_run_state: Option<bool>,
    #[yaserde(rename = "DeviceInfoIgnoreLoadedState")]
    pub device_info_ignore_loaded_state: Option<bool>,
    #[yaserde(rename = "DeviceCompareAllowCompatibleManufacturerId")]
    pub device_compare_allow_compatible_manufacturer_id: Option<bool>,
    #[yaserde(rename = "LineCoupler0912NewProgrammingStyle")]
    pub line_coupler0912_new_programming_style: Option<bool>,
    #[yaserde(rename = "MaxRoutingApduLength")]
    pub max_routing_apdu_length: Option<u32>,
    #[yaserde(rename = "Comparable")]
    pub comparable: Option<bool>,
    #[yaserde(rename = "Reconstructable")]
    pub reconstructable: Option<bool>,
    #[yaserde(rename = "DownloadInvisibleParameters")]
    pub download_invisible_parameters: Option<String>,
    #[yaserde(rename = "SupportsExtendedMemoryServices")]
    pub supports_extended_memory_services: Option<bool>,
    #[yaserde(rename = "SupportsExtendedPropertyServices")]
    pub supports_extended_property_services: Option<bool>,
    #[yaserde(rename = "SupportsIpSystemBroadcast")]
    pub supports_ip_system_broadcast: Option<bool>,
    #[yaserde(rename = "NotLoadable")]
    pub not_loadable: Option<String>,
    #[yaserde(rename = "NotLoadableMessageRef")]
    pub not_loadable_message_ref: Option<String>,
    #[yaserde(rename = "CustomerAdjustableParameters")]
    pub customer_adjustable_parameters: Option<String>,
    #[yaserde(rename = "MasterResetOnCRCMismatch")]
    pub master_reset_on_crc_mismatch: Option<bool>,
    #[yaserde(rename = "PromptBeforeFullDownload")]
    pub prompt_before_full_download: Option<bool>,
    #[yaserde(rename = "LegacyPatchManufacturerIdInTaskSegment")]
    pub legacy_patch_manufacturer_id_in_task_segment: Option<bool>,
}

// ApplicationProgramStatict ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramStatict {
    #[yaserde(rename = "Code")]
    pub code: Option<Code>,
    #[yaserde(rename = "ParameterTypes")]
    pub parameter_types: Option<ParameterTypes>,
    #[yaserde(rename = "Parameters")]
    pub parameters: Option<Parameters>,
    #[yaserde(rename = "ParameterRefs")]
    pub parameter_refs: Option<ParameterRefs>,
    #[yaserde(rename = "ParameterCalculations")]
    pub parameter_calculations: Option<ParameterCalculations>,
    #[yaserde(rename = "ParameterValidations")]
    pub parameter_validations: Option<ParameterValidations>,
    #[yaserde(rename = "ComObjectTable")]
    pub com_object_table: Option<ComObjectTable>,
    #[yaserde(rename = "ComObjectRefs")]
    pub com_object_refs: Option<ComObjectRefs>,
    #[yaserde(rename = "AddressTable")]
    pub address_table: Option<AddressTable>,
    #[yaserde(rename = "AssociationTable")]
    pub association_table: Option<AssociationTable>,
    #[yaserde(rename = "FixupList")]
    pub fixup_list: Option<FixupList>,
    #[yaserde(rename = "LoadProcedures")]
    pub load_procedures: Option<LoadProcedurest>,
    #[yaserde(rename = "Extension")]
    pub extension: Option<Extension>,
    #[yaserde(rename = "BinaryData")]
    pub binary_data: Option<BinaryData>,
    #[yaserde(rename = "DeviceCompare")]
    pub device_compare: Option<DeviceCompare>,
    #[yaserde(rename = "Messages")]
    pub messages: Option<Messages>,
    #[yaserde(rename = "Script")]
    pub script: Option<Script>,
    #[yaserde(rename = "SecurityRoles")]
    pub security_roles: Option<SecurityRoles>,
    #[yaserde(rename = "BusInterfaces")]
    pub bus_interfaces: Option<BusInterfaces>,
    #[yaserde(rename = "Options")]
    pub options: Option<Options>,
}

// ChannelIndependentBlock is registration-relevant list
#[derive(Debug, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub enum ChannelIndependentBlock {
    #[yaserde(rename = "ParameterBlock")]
    ParameterBlock(ComObjectParameterBlockt),
    #[yaserde(rename = "choose")]
    Choose(ChannelChooset),
    #[yaserde(rename = "BinaryDataRef")]
    BinaryDataReft(BinaryDataReft),
    #[yaserde(rename = "ComObjectRefRef")]
    ComObjectRefReft(ComObjectRefReft),
}

impl Default for ChannelIndependentBlock {
    fn default() -> ChannelIndependentBlock {
        ChannelIndependentBlock::ParameterBlock(Default::default())
    }
}

// ApplicationProgramDynamict is registration-relevant list
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramDynamict {
    #[yaserde(rename = "ChannelIndependentBlock")]
    pub channel_independent_block: Vec<ChannelIndependentBlock>,
    #[yaserde(rename = "Channel")]
    pub channel: Vec<ApplicationProgramChannelt>,
    #[yaserde(rename = "choose")]
    pub choose: Vec<DependentChannelChooset>,
}

// LoadProcedure ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LoadProcedure {
    #[yaserde(rename = "MergeId")]
    pub merge_id: Option<u16>,
    #[yaserde(flatten)]
    pub load_proceduret: LoadProceduret,
}

// LoadProcedurest ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LoadProcedurest {
    #[yaserde(rename = "LoadProcedure")]
    pub load_procedure: Vec<LoadProcedure>,
}

// LoadProceduret is registration-relevant list
#[derive(Debug, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub enum LoadProceduret {
    #[yaserde(rename = "knx:LdCtrlBase")]
    LdCtrlBase(LdCtrlBaset),
    #[yaserde(rename = "choose")]
    Choose(LdCtrlBaseChooset),
}

impl Default for LoadProceduret {
    fn default() -> LoadProceduret {
        LoadProceduret::LdCtrlBase(Default::default())
    }
}

// When is registration-relevant list
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct When {
    #[yaserde(rename = "knx:LdCtrlBase")]
    pub knx_ld_ctrl_base: Option<LdCtrlBaset>,
    #[yaserde(rename = "choose")]
    pub choose: Option<LdCtrlBaseChooset>,
    #[yaserde(flatten)]
    pub whent: Whent,
}

// LdCtrlBaseChooset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlBaseChooset {
    #[yaserde(attribute, rename = "ParamRefId")]
    pub param_ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "when")]
    pub when: Vec<When>,
}

// OnError ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct OnError {
    #[yaserde(rename = "Cause")]
    pub cause: String,
    #[yaserde(rename = "Ignore")]
    pub ignore: Option<bool>,
    #[yaserde(rename = "MessageRef")]
    pub message_ref: Option<String>,
}

// LdCtrlBaset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlBaset {
    #[yaserde(rename = "AppliesTo")]
    pub applies_to: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "OnError")]
    pub on_error: Vec<OnError>,
}

// LdCtrlCompareBaset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlCompareBaset {
    #[yaserde(rename = "AllowCachedValue")]
    pub allow_cached_value: Option<bool>,
    #[yaserde(rename = "InlineData")]
    pub inline_data: Option<String>,
    #[yaserde(rename = "Mask")]
    pub mask: Option<String>,
    #[yaserde(rename = "Range")]
    pub range: Option<String>,
    #[yaserde(rename = "Invert")]
    pub invert: Option<bool>,
    #[yaserde(rename = "RetryInterval")]
    pub retry_interval: Option<u16>,
    #[yaserde(rename = "TimeOut")]
    pub time_out: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// ld_ctrl_base ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ld_ctrl_base {
    #[yaserde(rename = "LdCtrlBase")]
    pub ld_ctrl_base: LdCtrlBaset,
}

// LdCtrlUnload ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlUnload {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlLoad ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlLoad {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlMaxLength ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlMaxLength {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlClearCachedObjectTypes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlClearCachedObjectTypes {
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlLoadCompleted ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlLoadCompleted {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlAbsSegment ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlAbsSegment {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "SegType")]
    pub seg_type: u16,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(rename = "Size")]
    pub size: u16,
    #[yaserde(rename = "Access")]
    pub access: u16,
    #[yaserde(rename = "MemType")]
    pub mem_type: u16,
    #[yaserde(rename = "SegFlags")]
    pub seg_flags: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlRelSegment ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlRelSegment {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "Mode")]
    pub mode: u16,
    #[yaserde(rename = "Fill")]
    pub fill: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlTaskSegment ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlTaskSegment {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlTaskPtr ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlTaskPtr {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "InitPtr")]
    pub init_ptr: u16,
    #[yaserde(rename = "SavePtr")]
    pub save_ptr: u16,
    #[yaserde(rename = "SerialPtr")]
    pub serial_ptr: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlTaskCtrl1 ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlTaskCtrl1 {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(rename = "Count")]
    pub count: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlTaskCtrl2 ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlTaskCtrl2 {
    #[yaserde(rename = "LsmIdx")]
    pub lsm_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Callback")]
    pub callback: u16,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(rename = "Seg0")]
    pub seg0: u16,
    #[yaserde(rename = "Seg1")]
    pub seg1: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlWriteProp ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlWriteProp {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(rename = "StartElement")]
    pub start_element: Option<u16>,
    #[yaserde(rename = "Count")]
    pub count: Option<u16>,
    #[yaserde(rename = "Verify")]
    pub verify: bool,
    #[yaserde(rename = "InlineData")]
    pub inline_data: Option<String>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlCompareProp ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlCompareProp {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(rename = "StartElement")]
    pub start_element: Option<u16>,
    #[yaserde(rename = "Count")]
    pub count: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_compare_baset: LdCtrlCompareBaset,
}

// LdCtrlLoadImageProp ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlLoadImageProp {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(rename = "Count")]
    pub count: Option<u16>,
    #[yaserde(rename = "StartElement")]
    pub start_element: Option<u16>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlInvokeFunctionProp ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlInvokeFunctionProp {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(rename = "InlineData")]
    pub inline_data: Option<String>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlReadFunctionProp ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlReadFunctionProp {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlWriteMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlWriteMem {
    #[yaserde(rename = "AddressSpace")]
    pub address_space: Option<String>,
    #[yaserde(rename = "Address")]
    pub address: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "Verify")]
    pub verify: bool,
    #[yaserde(rename = "InlineData")]
    pub inline_data: Option<String>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlCompareMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlCompareMem {
    #[yaserde(rename = "AddressSpace")]
    pub address_space: Option<String>,
    #[yaserde(rename = "Address")]
    pub address: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_compare_baset: LdCtrlCompareBaset,
}

// LdCtrlLoadImageMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlLoadImageMem {
    #[yaserde(rename = "AddressSpace")]
    pub address_space: Option<String>,
    #[yaserde(rename = "Address")]
    pub address: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlWriteRelMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlWriteRelMem {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(rename = "Verify")]
    pub verify: bool,
    #[yaserde(rename = "InlineData")]
    pub inline_data: Option<String>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlCompareRelMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlCompareRelMem {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_compare_baset: LdCtrlCompareBaset,
}

// LdCtrlLoadImageRelMem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlLoadImageRelMem {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "Offset")]
    pub offset: u32,
    #[yaserde(rename = "Size")]
    pub size: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlConnect ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlConnect {
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlDisconnect ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlDisconnect {
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlRestart ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlRestart {
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlMasterReset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlMasterReset {
    #[yaserde(rename = "EraseCode")]
    pub erase_code: u16,
    #[yaserde(rename = "ChannelNumber")]
    pub channel_number: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlDelay ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlDelay {
    #[yaserde(rename = "MilliSeconds")]
    pub milli_seconds: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlSetControlVariable ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlSetControlVariable {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Value")]
    pub value: bool,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlMapError ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlMapError {
    #[yaserde(rename = "LdCtrlFilter")]
    pub ld_ctrl_filter: Option<u16>,
    #[yaserde(rename = "OriginalError")]
    pub original_error: u32,
    #[yaserde(rename = "MappedError")]
    pub mapped_error: u32,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlProgressText ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlProgressText {
    #[yaserde(rename = "TextId")]
    pub text_id: Option<u32>,
    #[yaserde(rename = "MessageRef")]
    pub message_ref: Option<String>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlDeclarePropDesc ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlDeclarePropDesc {
    #[yaserde(rename = "ObjIdx")]
    pub obj_idx: Option<u16>,
    #[yaserde(rename = "ObjType")]
    pub obj_type: Option<u16>,
    #[yaserde(rename = "Occurrence")]
    pub occurrence: Option<u16>,
    #[yaserde(rename = "PropId")]
    pub prop_id: u16,
    #[yaserde(rename = "PropType")]
    pub prop_type: String,
    #[yaserde(rename = "MaxElements")]
    pub max_elements: u16,
    #[yaserde(rename = "ReadAccess")]
    pub read_access: u16,
    #[yaserde(rename = "WriteAccess")]
    pub write_access: u16,
    #[yaserde(rename = "Writable")]
    pub writable: bool,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlClearLCFilterTable ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlClearLCFilterTable {
    #[yaserde(rename = "UseFunctionProp")]
    pub use_function_prop: Option<bool>,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// LdCtrlMerge ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LdCtrlMerge {
    #[yaserde(rename = "MergeId")]
    pub merge_id: u16,
    #[yaserde(flatten)]
    pub ld_ctrl_baset: LdCtrlBaset,
}

// Fixupt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Fixupt {
    #[yaserde(rename = "FunctionRef")]
    pub function_ref: String,
    #[yaserde(rename = "CodeSegment")]
    pub code_segment: String,
    #[yaserde(rename = "Offset")]
    pub offset: Vec<u32>,
}

// BinaryDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BinaryDatat {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Data")]
    pub data: String,
}

// TypeNumber ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeNumber {
    #[yaserde(attribute, rename = "SizeInBit")]
    pub size_in_bit: u16,
    #[yaserde(attribute, rename = "Type")]
    pub type_attr: String,
    #[yaserde(attribute, rename = "minInclusive")]
    pub min_inclusive: i64,
    #[yaserde(attribute, rename = "maxInclusive")]
    pub max_inclusive: i64,
    #[yaserde(rename = "Increment")]
    pub increment: Option<i64>,
    #[yaserde(rename = "UIHint")]
    pub ui_hint: Option<String>,
}

// TypeFloat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeFloat {
    #[yaserde(attribute, rename = "Encoding")]
    pub encoding: String,
    #[yaserde(attribute, rename = "minInclusive")]
    pub min_inclusive: f64,
    #[yaserde(attribute, rename = "maxInclusive")]
    pub max_inclusive: f64,
    #[yaserde(rename = "Increment")]
    pub increment: Option<f64>,
    #[yaserde(rename = "UIHint")]
    pub ui_hint: Option<String>,
    #[yaserde(rename = "DisplayFormat")]
    pub display_format: Option<String>,
}

// TypeRestriction ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeRestriction {
    #[yaserde(attribute, rename = "Base")]
    pub base: String,
    #[yaserde(attribute, rename = "SizeInBit")]
    pub size_in_bit: u16,
    #[yaserde(attribute, rename = "Offset")]
    pub offset: Vec<u32>,
    #[yaserde(rename = "Enumeration")]
    pub enumeration: Vec<TypeRestrictionEnumeration>,
}

#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeRestrictionEnumeration {
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(attribute, rename = "Value")]
    pub value: i64,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
}

// TypeText ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeText {
    #[yaserde(attribute, rename = "SizeInBit")]
    pub size_in_bit: u16,
    #[yaserde(rename = "Pattern")]
    pub pattern: Option<String>,
}

// TypeTime ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeTime {
    #[yaserde(rattribute, ename = "SizeInBit")]
    pub size_in_bit: u16,
    #[yaserde(attribute, rename = "Unit")]
    pub unit: String,
    #[yaserde(attribute, rename = "minInclusive")]
    pub min_inclusive: i64,
    #[yaserde(attribute, rename = "maxInclusive")]
    pub max_inclusive: i64,
    #[yaserde(rename = "UIHint")]
    pub ui_hint: Option<String>,
}

// TypeDate ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeDate {
    #[yaserde(attribute, rename = "Encoding")]
    pub encoding: String,
    #[yaserde(rename = "DisplayTheYear")]
    pub display_the_year: Option<bool>,
}

// TypeIPAddress ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeIPAddress {
    #[yaserde(rename = "AddressType")]
    pub address_type: String,
    #[yaserde(rename = "Version")]
    pub version: Option<String>,
}

// TypePicture ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypePicture {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "HorizontalAlignment")]
    pub horizontal_alignment: Option<String>,
}

// TypeColor ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeColor {
    #[yaserde(attribute, rename = "Space")]
    pub space: String,
}

// TypeRawData ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TypeRawData {
    #[yaserde(attribute, rename = "MaxSize")]
    pub max_size: u16,
}

// ParameterTypet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterTypet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Plugin")]
    pub plugin: Option<String>,
    #[yaserde(rename = "ValidationErrorRef")]
    pub validation_error_ref: Option<String>,
    #[yaserde(rename = "TypeNumber")]
    pub type_number: Option<TypeNumber>,
    #[yaserde(rename = "TypeFloat")]
    pub type_float: Option<TypeFloat>,
    #[yaserde(rename = "TypeRestriction")]
    pub type_restriction: Option<TypeRestriction>,
    #[yaserde(rename = "TypeText")]
    pub type_text: Option<TypeText>,
    #[yaserde(rename = "TypeTime")]
    pub type_time: Option<TypeTime>,
    #[yaserde(rename = "TypeDate")]
    pub type_date: Option<TypeDate>,
    #[yaserde(rename = "TypeIPAddress")]
    pub type_ip_address: Option<TypeIPAddress>,
    #[yaserde(rename = "TypePicture")]
    pub type_picture: Option<TypePicture>,
    #[yaserde(rename = "TypeColor")]
    pub type_color: Option<TypeColor>,
    #[yaserde(rename = "TypeRawData")]
    pub type_raw_data: Option<TypeRawData>,
    // #[yaserde(rename = "TypeNone")]
    // pub type_none: TypeNone,
}

// Parametert ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Parametert {
    #[yaserde(rename = "LegacyPatchAlways")]
    pub legacy_patch_always: Option<bool>,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "ParameterType")]
    pub parameter_type: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "SuffixText")]
    pub suffix_text: Option<String>,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "Value")]
    pub value: String,
    #[yaserde(rename = "InitialValue")]
    pub initial_value: Option<String>,
    #[yaserde(rename = "CustomerAdjustable")]
    pub customer_adjustable: Option<bool>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Memory")]
    pub memory: Memory,
    #[yaserde(rename = "Property")]
    pub property: Property,
}

// UnionParametert ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct UnionParametert {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "ParameterType")]
    pub parameter_type: String,
    #[yaserde(rename = "Offset")]
    pub offset: u16,
    #[yaserde(rename = "BitOffset")]
    pub bit_offset: u16,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "SuffixText")]
    pub suffix_text: Option<String>,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "Value")]
    pub value: String,
    #[yaserde(rename = "InitialValue")]
    pub initial_value: Option<String>,
    #[yaserde(rename = "CustomerAdjustable")]
    pub customer_adjustable: Option<bool>,
    #[yaserde(rename = "DefaultUnionParameter")]
    pub default_union_parameter: Option<bool>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// ParameterReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterReft {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "SuffixText")]
    pub suffix_text: Option<String>,
    #[yaserde(rename = "Tag")]
    pub tag: Option<String>,
    #[yaserde(rename = "DisplayOrder")]
    pub display_order: Option<i32>,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "Value")]
    pub value: Option<String>,
    #[yaserde(rename = "InitialValue")]
    pub initial_value: Option<String>,
    #[yaserde(rename = "CustomerAdjustable")]
    pub customer_adjustable: Option<bool>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "ForbidGrantingUseByCustomer")]
    pub forbid_granting_use_by_customer: Option<bool>,
}

// ParameterRefRef ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterRefRef {
    #[yaserde(rename = "AliasName")]
    pub alias_name: Option<String>,
    #[yaserde(flatten)]
    pub parameter_ref_reft: ParameterRefReft,
}

// LParameters ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LParameters {
    #[yaserde(rename = "ParameterRefRef")]
    pub parameter_ref_ref: Vec<ParameterRefRef>,
}

// RParameters ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RParameters {
    #[yaserde(rename = "ParameterRefRef")]
    pub parameter_ref_ref: Vec<ParameterRefRef>,
}

// ParameterCalculationt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterCalculationt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Language")]
    pub language: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(attribute, rename = "RLTransformationFunc")]
    pub rl_transformation_func: Option<String>,
    #[yaserde(attribute, rename = "RLTransformationParameters")]
    pub rl_transformation_parameters: Option<String>,
    #[yaserde(attribute, rename = "LRTransformationFunc")]
    pub lr_transformation_func: Option<String>,
    #[yaserde(attribute, rename = "LRTransformationParameters")]
    pub lr_transformation_parameters: Option<String>,
    #[yaserde(attribute, rename = "RLTransformation")]
    pub rl_transformation: Option<String>,
    #[yaserde(attribute, rename = "LRTransformation")]
    pub lr_transformation: Option<String>,
    #[yaserde(attribute, rename = "LParameters")]
    pub l_parameters: Option<LParameters>,
    #[yaserde(attribute, rename = "TypeNone")]
    pub type_none: Option<String>,
}

// ParameterValidationt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterValidationt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "ValidationFunc")]
    pub validation_func: String,
    #[yaserde(rename = "ValidationParameters")]
    pub validation_parameters: Option<String>,
    #[yaserde(rename = "Parameters")]
    pub parameters: Parameters,
}

// ComObjectt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: u32,
    #[yaserde(attribute, rename = "FunctionText")]
    pub function_text: String,
    #[yaserde(attribute, rename = "Priority")]
    pub priority: Option<String>,
    #[yaserde(attribute, rename = "ObjectSize")]
    pub object_size: String,
    #[yaserde(attribute, rename = "ReadFlag")]
    pub read_flag: String,
    #[yaserde(attribute, rename = "WriteFlag")]
    pub write_flag: String,
    #[yaserde(attribute, rename = "CommunicationFlag")]
    pub communication_flag: String,
    #[yaserde(attribute, rename = "TransmitFlag")]
    pub transmit_flag: String,
    #[yaserde(attribute, rename = "UpdateFlag")]
    pub update_flag: String,
    #[yaserde(attribute, rename = "ReadOnInitFlag")]
    pub read_on_init_flag: String,
    #[yaserde(attribute, rename = "DatapointType")]
    pub datapoint_type: Option<String>,
    #[yaserde(attribute, rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(attribute, rename = "SecurityRequired")]
    pub security_required: Option<String>,
}

// ApplicationProgramChannelt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramChannelt {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(attribute, rename = "Number")]
    pub number: String,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Icon")]
    pub icon: Option<String>,
    #[yaserde(rename = "HelpContext")]
    pub help_context: Option<String>,
    #[yaserde(rename = "ParameterBlock")]
    pub parameter_block: Vec<ComObjectParameterBlockt>,
    #[yaserde(rename = "ComObjectRefRef")]
    pub com_object_ref_ref: Vec<ComObjectRefReft>,
    #[yaserde(rename = "BinaryDataRef")]
    pub binary_data_ref: Vec<BinaryDataReft>,
    #[yaserde(rename = "choose")]
    pub choose: Vec<ChannelChooset>,
}

// Whent ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Whent {
    #[yaserde(rename = "test")]
    pub test: Option<String>,
    #[yaserde(rename = "default")]
    pub default: Option<bool>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// ComObjectParameterChooset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectParameterChooset {
    #[yaserde(attribute, rename = "ParamRefId")]
    pub param_ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "when")]
    pub when: Vec<When>,
}

// Row ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Row {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "CollapseIfEmpty")]
    pub collapse_if_empty: Option<bool>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// Rows ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Rows {
    #[yaserde(rename = "Row")]
    pub row: Row,
}

// Column ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Column {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(attribute, rename = "Width")]
    pub width: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// Columns ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Columns {
    #[yaserde(rename = "Column")]
    pub column: Column,
}

// ComObjectParameterBlockt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectParameterBlockt {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "HelpTopic")]
    pub help_topic: Option<u32>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(attribute, rename = "ParamRefId")]
    pub param_ref_id: Option<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "Inline")]
    pub inline: Option<bool>,
    #[yaserde(rename = "Layout")]
    pub layout: Option<String>,
    #[yaserde(rename = "Cell")]
    pub cell: Option<String>,
    #[yaserde(rename = "Icon")]
    pub icon: Option<String>,
    #[yaserde(rename = "HelpContext")]
    pub help_context: Option<String>,
    #[yaserde(rename = "ShowInComObjectTree")]
    pub show_in_com_object_tree: Option<bool>,
    #[yaserde(rename = "Rows")]
    pub rows: Option<Rows>,
    #[yaserde(rename = "Columns")]
    pub columns: Option<Columns>,
    #[yaserde(rename = "ParameterBlock")]
    pub parameter_block: Vec<ComObjectParameterBlockt>,
    #[yaserde(rename = "ParameterSeparator")]
    pub parameter_separator: Vec<ParameterSeparatort>,
    #[yaserde(rename = "ParameterRefRef")]
    pub parameter_ref_ref: Vec<ParameterRefReft>,
    #[yaserde(rename = "Button")]
    pub button: Vec<Buttont>,
    #[yaserde(rename = "choose")]
    pub choose: Vec<ComObjectParameterChooset>,
    #[yaserde(rename = "BinaryDataRef")]
    pub binary_data_ref: Vec<BinaryDataReft>,
    #[yaserde(rename = "ComObjectRefRef")]
    pub com_object_ref_ref: Vec<ComObjectRefReft>,
    #[yaserde(rename = "Assign")]
    pub assign: Vec<Assignt>,
    #[yaserde(rename = "Channel")]
    pub channel: Vec<ApplicationProgramChannelt>,
}

// BinaryDataReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BinaryDataReft {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Data")]
    pub data: String,
}

// ComObjectReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectReft {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "Tag")]
    pub tag: Option<String>,
    #[yaserde(rename = "FunctionText")]
    pub function_text: Option<String>,
    #[yaserde(rename = "Priority")]
    pub priority: Option<String>,
    #[yaserde(rename = "ObjectSize")]
    pub object_size: Option<String>,
    #[yaserde(rename = "ReadFlag")]
    pub read_flag: Option<String>,
    #[yaserde(rename = "WriteFlag")]
    pub write_flag: Option<String>,
    #[yaserde(rename = "CommunicationFlag")]
    pub communication_flag: Option<String>,
    #[yaserde(rename = "TransmitFlag")]
    pub transmit_flag: Option<String>,
    #[yaserde(rename = "UpdateFlag")]
    pub update_flag: Option<String>,
    #[yaserde(rename = "ReadOnInitFlag")]
    pub read_on_init_flag: Option<String>,
    #[yaserde(rename = "DatapointType")]
    pub datapoint_type: Vec<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Roles")]
    pub roles: Option<String>,
    #[yaserde(rename = "SecurityRequired")]
    pub security_required: Option<String>,
}

// ComObjectRefReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectRefReft {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// ParameterRefReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterRefReft {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "HelpContext")]
    pub help_context: Option<String>,
    #[yaserde(rename = "IndentLevel")]
    pub indent_level: Option<u16>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Cell")]
    pub cell: Option<String>,
    #[yaserde(rename = "Icon")]
    pub icon: Option<String>,
}

// ParameterSeparatort ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterSeparatort {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "UIHint")]
    pub ui_hint: Option<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Cell")]
    pub cell: Option<String>,
    #[yaserde(rename = "Icon")]
    pub icon: Option<String>,
}

// Buttont ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Buttont {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Access")]
    pub access: Option<String>,
    #[yaserde(rename = "TextParameterRefId")]
    pub text_parameter_ref_id: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Cell")]
    pub cell: Option<String>,
    #[yaserde(rename = "Icon")]
    pub icon: Option<String>,
    #[yaserde(rename = "EventHandler")]
    pub event_handler: Option<String>,
    #[yaserde(rename = "EventHandlerParameters")]
    pub event_handler_parameters: Option<String>,
    #[yaserde(rename = "EventHandlerOnline")]
    pub event_handler_online: Option<String>,
}

// Assignt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Assignt {
    #[yaserde(rename = "TargetParamRefRef")]
    pub target_param_ref_ref: String,
    #[yaserde(rename = "SourceParamRefRef")]
    pub source_param_ref_ref: Option<String>,
    #[yaserde(rename = "Value")]
    pub value: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// ChannelChooset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ChannelChooset {
    #[yaserde(attribute, rename = "ParamRefId")]
    pub param_ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "when")]
    pub when: Vec<When>,
}

// DependentChannelChooset ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DependentChannelChooset {
    #[yaserde(attribute, rename = "ParamRefId")]
    pub param_ref_id: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "when")]
    pub when: Vec<When>,
}

// Renamet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Renamet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// Attribute ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Attribute {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Value")]
    pub value: String,
}

// Attributes ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Attributes {
    #[yaserde(rename = "Attribute")]
    pub attribute: Vec<Attribute>,
}

// Product ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Product {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "OrderNumber")]
    pub order_number: String,
    #[yaserde(rename = "IsRailMounted")]
    pub is_rail_mounted: bool,
    #[yaserde(rename = "WidthInMillimeter")]
    pub width_in_millimeter: Option<f64>,
    #[yaserde(rename = "VisibleDescription")]
    pub visible_description: Option<String>,
    #[yaserde(rename = "DefaultLanguage")]
    pub default_language: Option<String>,
    #[yaserde(rename = "NonRegRelevantDataVersion")]
    pub non_reg_relevant_data_version: Option<u16>,
    #[yaserde(rename = "Hash")]
    pub hash: Option<String>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Baggages")]
    pub baggages: Baggages,
    #[yaserde(rename = "Attributes")]
    pub attributes: Attributes,
    #[yaserde(rename = "RegistrationInfo")]
    pub registration_info: RegistrationInfot,
}

// Products ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Products {
    #[yaserde(rename = "Product")]
    pub product: Vec<Product>,
}

// Hardware2Programs ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Hardware2Programs {
    #[yaserde(rename = "Hardware2Program")]
    pub hardware2_program: Vec<Hardware2Programt>,
}

// Hardwaret ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Hardwaret {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "SerialNumber")]
    pub serial_number: String,
    #[yaserde(rename = "VersionNumber")]
    pub version_number: u16,
    #[yaserde(rename = "BusCurrent")]
    pub bus_current: Option<f64>,
    #[yaserde(rename = "IsAccessory")]
    pub is_accessory: Option<bool>,
    #[yaserde(rename = "HasIndividualAddress")]
    pub has_individual_address: bool,
    #[yaserde(rename = "HasApplicationProgram")]
    pub has_application_program: bool,
    #[yaserde(rename = "HasApplicationProgram2")]
    pub has_application_program2: Option<bool>,
    #[yaserde(rename = "IsPowerSupply")]
    pub is_power_supply: Option<bool>,
    #[yaserde(rename = "IsChoke")]
    pub is_choke: Option<bool>,
    #[yaserde(rename = "IsCoupler")]
    pub is_coupler: Option<bool>,
    #[yaserde(rename = "IsPowerLineRepeater")]
    pub is_power_line_repeater: Option<bool>,
    #[yaserde(rename = "IsPowerLineSignalFilter")]
    pub is_power_line_signal_filter: Option<bool>,
    #[yaserde(rename = "IsCable")]
    pub is_cable: Option<bool>,
    #[yaserde(rename = "IsIPEnabled")]
    pub is_ip_enabled: Option<bool>,
    #[yaserde(rename = "IsRFRetransmitter")]
    pub is_rf_retransmitter: Option<bool>,
    #[yaserde(rename = "RuntimeUnidirectional")]
    pub runtime_unidirectional: Option<bool>,
    #[yaserde(rename = "OriginalManufacturer")]
    pub original_manufacturer: Option<String>,
    #[yaserde(rename = "RFDeviceMode")]
    pub rf_device_mode: Option<String>,
    #[yaserde(rename = "NoDownloadWithoutPlugin")]
    pub no_download_without_plugin: Option<bool>,
    #[yaserde(rename = "NonRegRelevantDataVersion")]
    pub non_reg_relevant_data_version: Option<u16>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "Products")]
    pub products: Products,
    #[yaserde(rename = "Hardware2Programs")]
    pub hardware2_programs: Hardware2Programs,
}

// Hardware2Programt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Hardware2Programt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "MediumTypes")]
    pub medium_types: Vec<String>,
    #[yaserde(rename = "Hash")]
    pub hash: Option<String>,
    #[yaserde(rename = "CheckSums")]
    pub check_sums: Option<String>,
    #[yaserde(rename = "LoadedImage")]
    pub loaded_image: Option<String>,
    #[yaserde(rename = "ApplicationProgramRef")]
    pub application_program_ref: Vec<ApplicationProgramReft>,
    #[yaserde(rename = "RegistrationInfo")]
    pub registration_info: RegistrationInfot,
}

// ApplicationProgramReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ApplicationProgramReft {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
}

// RegistrationInfot ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct RegistrationInfot {
    #[yaserde(rename = "RegistrationStatus")]
    pub registration_status: String,
    #[yaserde(rename = "RegistrationNumber")]
    pub registration_number: Option<String>,
    #[yaserde(rename = "OriginalRegistrationNumber")]
    pub original_registration_number: Option<String>,
    #[yaserde(rename = "RegistrationDate")]
    pub registration_date: Option<u16>,
    #[yaserde(rename = "RegistrationSignature")]
    pub registration_signature: Option<String>,
    #[yaserde(rename = "RegistrationKey")]
    pub registration_key: Option<String>,
}

// CatalogItem ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct CatalogItem {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: i32,
    #[yaserde(rename = "VisibleDescription")]
    pub visible_description: Option<String>,
    #[yaserde(rename = "ProductRefId")]
    pub product_ref_id: String,
    #[yaserde(rename = "Hardware2ProgramRefId")]
    pub hardware2_program_ref_id: Option<String>,
    #[yaserde(rename = "DefaultLanguage")]
    pub default_language: Option<String>,
    #[yaserde(rename = "NonRegRelevantDataVersion")]
    pub non_reg_relevant_data_version: Option<u16>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
}

// CatalogSectiont ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct CatalogSectiont {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: String,
    #[yaserde(rename = "VisibleDescription")]
    pub visible_description: Option<String>,
    #[yaserde(rename = "DefaultLanguage")]
    pub default_language: Option<String>,
    #[yaserde(rename = "NonRegRelevantDataVersion")]
    pub non_reg_relevant_data_version: Option<u16>,
    #[yaserde(rename = "InternalDescription")]
    pub internal_description: Option<String>,
    #[yaserde(rename = "CatalogSection")]
    pub catalog_section: Vec<CatalogSectiont>,
    #[yaserde(rename = "CatalogItem")]
    pub catalog_item: Vec<CatalogItem>,
}

// Translation ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Translation {
    #[yaserde(attribute, rename = "AttributeName")]
    pub attribute_name: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
}

// TranslationElement ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TranslationElement {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "Translation")]
    pub translation: Vec<Translation>,
}

// TranslationUnit ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct TranslationUnit {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "Version")]
    pub version: Option<i32>,
    #[yaserde(rename = "TranslationElement")]
    pub translation_element: Vec<TranslationElement>,
}

// LanguageDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct LanguageDatat {
    #[yaserde(attribute, rename = "Identifier")]
    pub identifier: String,
    #[yaserde(rename = "TranslationUnit")]
    pub translation_unit: Vec<TranslationUnit>,
}

// HistoryEntry ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct HistoryEntry {
    #[yaserde(rename = "Date")]
    pub date: u16,
    #[yaserde(rename = "User")]
    pub user: Option<String>,
    #[yaserde(attribute, rename = "Text")]
    pub text: String,
    #[yaserde(rename = "Detail")]
    pub detail: Option<String>,
}

// HistoryEntries ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct HistoryEntries {
    #[yaserde(rename = "HistoryEntry")]
    pub history_entry: Vec<HistoryEntry>,
}

// ToDoItems ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ToDoItems {
    #[yaserde(rename = "ToDoItem")]
    pub to_do_item: Vec<ToDoItemt>,
}

// ProjectTraces ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProjectTraces {
    #[yaserde(rename = "ProjectTrace")]
    pub project_trace: Vec<ProjectTracet>,
}

// DeviceCertificates ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeviceCertificates {
    #[yaserde(rename = "DeviceCertificate")]
    pub device_certificate: Vec<DeviceCertificatet>,
}

// ProjectInformation ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProjectInformation {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "GroupAddressStyle")]
    pub group_address_style: String,
    #[yaserde(rename = "ProjectNumber")]
    pub project_number: Option<String>,
    #[yaserde(rename = "ContractNumber")]
    pub contract_number: Option<String>,
    #[yaserde(rename = "LastModified")]
    pub last_modified: Option<u16>,
    #[yaserde(rename = "ProjectStart")]
    pub project_start: Option<u16>,
    #[yaserde(rename = "ProjectEnd")]
    pub project_end: Option<u16>,
    #[yaserde(rename = "ProjectId")]
    pub project_id: Option<u16>,
    #[yaserde(rename = "ProjectPassword")]
    pub project_password: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "ProjectTracingLevel")]
    pub project_tracing_level: Option<String>,
    #[yaserde(rename = "ProjectTracingPassword")]
    pub project_tracing_password: Option<String>,
    #[yaserde(rename = "Hide16BitGroupsFromLegacyPlugins")]
    pub hide16_bit_groups_from_legacy_plugins: Option<bool>,
    #[yaserde(rename = "CodePage")]
    pub code_page: Option<String>,
    #[yaserde(rename = "BusAccessLegacyMode")]
    pub bus_access_legacy_mode: Option<bool>,
    #[yaserde(rename = "Guid")]
    pub guid: String,
    #[yaserde(rename = "LastUsedPuid")]
    pub last_used_puid: i32,
    #[yaserde(rename = "HistoryEntries")]
    pub history_entries: HistoryEntries,
    #[yaserde(rename = "ToDoItems")]
    pub to_do_items: ToDoItems,
    #[yaserde(rename = "ProjectTraces")]
    pub project_traces: ProjectTraces,
    #[yaserde(rename = "TypeNone")]
    pub type_none: u16,
}

// Installation ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Installation {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "InstallationId")]
    pub installation_id: Option<u16>,
    #[yaserde(rename = "BCUKey")]
    pub bcu_key: Option<u64>,
    #[yaserde(rename = "IPRoutingMulticastAddress")]
    pub ip_routing_multicast_address: Option<String>,
    #[yaserde(rename = "MulticastTTL")]
    pub multicast_ttl: Option<u16>,
    #[yaserde(rename = "IPRoutingBackboneKey")]
    pub ip_routing_backbone_key: Option<String>,
    #[yaserde(rename = "IPRoutingLatencyTolerance")]
    pub ip_routing_latency_tolerance: Option<u16>,
    #[yaserde(rename = "IPSyncLatencyFraction")]
    pub ip_sync_latency_fraction: Option<f64>,
    #[yaserde(rename = "DefaultLine")]
    pub default_line: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "IPRoutingBackboneSecurity")]
    pub ip_routing_backbone_security: Option<String>,
    #[yaserde(rename = "SplitType")]
    pub split_type: Option<String>,
    #[yaserde(rename = "Topology")]
    pub topology: Topologyt,
    #[yaserde(rename = "Locations")]
    pub locations: Locationst,
    #[yaserde(rename = "GroupAddresses")]
    pub group_addresses: GroupAddressest,
    #[yaserde(rename = "Trades")]
    pub trades: Tradest,
    #[yaserde(rename = "BusAccess")]
    pub bus_access: BusAccesst,
    #[yaserde(rename = "TypeNone")]
    pub type_none: String,
}

// Installations ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Installations {
    #[yaserde(rename = "Installation")]
    pub installation: Vec<Installation>,
}

// UserFiles ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct UserFiles {
    #[yaserde(rename = "UserFile")]
    pub user_file: Vec<UserFilet>,
}

// AddinData ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AddinData {
    #[yaserde(rename = "AddinData")]
    pub addin_data: Vec<AddinDatat>,
}

// Projectt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Projectt {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "ProjectInformation")]
    pub project_information: ProjectInformation,
    #[yaserde(rename = "Installations")]
    pub installations: Installations,
    #[yaserde(rename = "UserFiles")]
    pub user_files: UserFiles,
    #[yaserde(rename = "AddinData")]
    pub addin_data: AddinData,
}

// SplitInfost ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SplitInfost {
    #[yaserde(rename = "SplitInfo")]
    pub split_info: Vec<SplitInfot>,
}

// SplitInfot ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct SplitInfot {
    #[yaserde(rename = "ObjectPath")]
    pub object_path: String,
    #[yaserde(rename = "Cookie")]
    pub cookie: String,
}

// GroupAddress ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupAddress {
    #[yaserde(rename = "Address")]
    pub address: u16,
}

// AdditionalGroupAddresses ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AdditionalGroupAddresses {
    #[yaserde(rename = "GroupAddress")]
    pub group_address: GroupAddress,
}

// Line ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Line {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(rename = "MediumTypeRefId")]
    pub medium_type_ref_id: String,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "DomainAddress")]
    pub domain_address: Option<u64>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "DeviceInstance")]
    pub device_instance: Vec<DeviceInstancet>,
    #[yaserde(rename = "BusAccess")]
    pub bus_access: BusAccesst,
    #[yaserde(rename = "TypeNone")]
    pub type_none: i32,
}

// Area ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Area {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "TypeNone")]
    pub type_none: i32,
}

// UnassignedDevices ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct UnassignedDevices {
    #[yaserde(rename = "DeviceInstance")]
    pub device_instance: Vec<DeviceInstancet>,
}

// Topologyt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Topologyt {
    #[yaserde(rename = "Area")]
    pub area: Vec<Area>,
    #[yaserde(rename = "UnassignedDevices")]
    pub unassigned_devices: UnassignedDevices,
}

// ParameterInstanceRefs ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterInstanceRefs {
    #[yaserde(rename = "ParameterInstanceRef")]
    pub parameter_instance_ref: Vec<ParameterInstanceReft>,
}

// ComObjectInstanceRefs ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectInstanceRefs {
    #[yaserde(rename = "ComObjectInstanceRef")]
    pub com_object_instance_ref: Vec<ComObjectInstanceReft>,
}

// ChannelInstances ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ChannelInstances {
    #[yaserde(rename = "ChannelInstance")]
    pub channel_instance: Vec<ChannelInstancet>,
}

// Address ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Address {
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
}

// AdditionalAddresses ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AdditionalAddresses {
    #[yaserde(rename = "Address")]
    pub address: Vec<Address>,
}

// DeviceInstancet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeviceInstancet {
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "ProductRefId")]
    pub product_ref_id: String,
    #[yaserde(rename = "Hardware2ProgramRefId")]
    pub hardware2_program_ref_id: Option<String>,
    #[yaserde(rename = "Address")]
    pub address: Option<u16>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "LastModified")]
    pub last_modified: Option<u16>,
    #[yaserde(rename = "LastDownload")]
    pub last_download: Option<u16>,
    #[yaserde(rename = "LastUsedAPDULength")]
    pub last_used_apdu_length: Option<u16>,
    #[yaserde(rename = "ReadMaxAPDULength")]
    pub read_max_apdu_length: Option<u16>,
    #[yaserde(rename = "ReadMaxRoutingAPDULength")]
    pub read_max_routing_apdu_length: Option<u16>,
    #[yaserde(rename = "InstallationHints")]
    pub installation_hints: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "IndividualAddressLoaded")]
    pub individual_address_loaded: Option<bool>,
    #[yaserde(rename = "ApplicationProgramLoaded")]
    pub application_program_loaded: Option<bool>,
    #[yaserde(rename = "ParametersLoaded")]
    pub parameters_loaded: Option<bool>,
    #[yaserde(rename = "CommunicationPartLoaded")]
    pub communication_part_loaded: Option<bool>,
    #[yaserde(rename = "MediumConfigLoaded")]
    pub medium_config_loaded: Option<bool>,
    #[yaserde(rename = "LoadedImage")]
    pub loaded_image: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "CheckSums")]
    pub check_sums: Option<String>,
    #[yaserde(rename = "DownloadCounter")]
    pub download_counter: Option<u32>,
    #[yaserde(rename = "IsCommunicationObjectVisibilityCalculated")]
    pub is_communication_object_visibility_calculated: Option<bool>,
    #[yaserde(rename = "Broken")]
    pub broken: Option<bool>,
    #[yaserde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[yaserde(rename = "UniqueId")]
    pub unique_id: Option<String>,
    #[yaserde(rename = "IsRFRetransmitter")]
    pub is_rf_retransmitter: Option<bool>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "ParameterInstanceRefs")]
    pub parameter_instance_refs: ParameterInstanceRefs,
    #[yaserde(rename = "ComObjectInstanceRefs")]
    pub com_object_instance_refs: ComObjectInstanceRefs,
    #[yaserde(rename = "ChannelInstances")]
    pub channel_instances: ChannelInstances,
    #[yaserde(rename = "AdditionalAddresses")]
    pub additional_addresses: AdditionalAddresses,
    #[yaserde(rename = "BinaryData")]
    pub binary_data: BinaryData,
    #[yaserde(rename = "IPConfig")]
    pub ip_config: IPConfigt,
    #[yaserde(rename = "Security")]
    pub security: Securityt,
    #[yaserde(rename = "TypeNone")]
    pub type_none: i32,
}

// Role ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Role {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "Address")]
    pub address: u16,
}

// Securityt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Securityt {
    #[yaserde(rename = "LoadedIPRoutingBackboneKey")]
    pub loaded_ip_routing_backbone_key: Option<String>,
    #[yaserde(rename = "DeviceAuthenticationCode")]
    pub device_authentication_code: Option<String>,
    #[yaserde(rename = "DeviceAuthenticationCodeHash")]
    pub device_authentication_code_hash: Option<String>,
    #[yaserde(rename = "LoadedDeviceAuthenticationCodeHash")]
    pub loaded_device_authentication_code_hash: Option<String>,
    #[yaserde(rename = "DeviceManagementPassword")]
    pub device_management_password: Option<String>,
    #[yaserde(rename = "DeviceManagementPasswordHash")]
    pub device_management_password_hash: Option<String>,
    #[yaserde(rename = "LoadedDeviceManagementPasswordHash")]
    pub loaded_device_management_password_hash: Option<String>,
    #[yaserde(rename = "ToolKey")]
    pub tool_key: Option<String>,
    #[yaserde(rename = "LoadedToolKey")]
    pub loaded_tool_key: Option<String>,
    #[yaserde(rename = "SequenceNumber")]
    pub sequence_number: Option<u64>,
    #[yaserde(rename = "SequenceNumberTimestamp")]
    pub sequence_number_timestamp: Option<u16>,
    #[yaserde(rename = "Role")]
    pub role: Role,
}

// DeviceInstanceReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeviceInstanceReft {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
}

// ParameterInstanceReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ParameterInstanceReft {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "Value")]
    pub value: Option<String>,
    #[yaserde(rename = "GrantUseByCustomer")]
    pub grant_use_by_customer: Option<bool>,
    #[yaserde(rename = "CustomizedText")]
    pub customized_text: Option<String>,
}

// ChannelInstancet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ChannelInstancet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "IsActive")]
    pub is_active: Option<bool>,
}

// Send ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Send {
    #[yaserde(rename = "GroupAddressRefId")]
    pub group_address_ref_id: String,
    #[yaserde(rename = "Acknowledge")]
    pub acknowledge: Option<bool>,
}

// Receive ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Receive {
    #[yaserde(rename = "GroupAddressRefId")]
    pub group_address_ref_id: String,
    #[yaserde(rename = "Acknowledge")]
    pub acknowledge: Option<bool>,
}

// Connectors ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Connectors {
    #[yaserde(rename = "Send")]
    pub send: Send,
    #[yaserde(rename = "Receive")]
    pub receive: Vec<Receive>,
}

// ComObjectInstanceReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ComObjectInstanceReft {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(attribute, rename = "Text")]
    pub text: Option<String>,
    #[yaserde(rename = "FunctionText")]
    pub function_text: Option<String>,
    #[yaserde(rename = "Priority")]
    pub priority: Option<String>,
    #[yaserde(rename = "ReadFlag")]
    pub read_flag: Option<String>,
    #[yaserde(rename = "WriteFlag")]
    pub write_flag: Option<String>,
    #[yaserde(rename = "CommunicationFlag")]
    pub communication_flag: Option<String>,
    #[yaserde(rename = "TransmitFlag")]
    pub transmit_flag: Option<String>,
    #[yaserde(rename = "UpdateFlag")]
    pub update_flag: Option<String>,
    #[yaserde(rename = "ReadOnInitFlag")]
    pub read_on_init_flag: Option<String>,
    #[yaserde(rename = "DatapointType")]
    pub datapoint_type: Vec<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "IsActive")]
    pub is_active: Option<bool>,
    #[yaserde(rename = "ChannelId")]
    pub channel_id: Option<String>,
    #[yaserde(rename = "Connectors")]
    pub connectors: Connectors,
}

// GroupRanget ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupRanget {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "RangeStart")]
    pub range_start: u16,
    #[yaserde(rename = "RangeEnd")]
    pub range_end: u16,
    #[yaserde(rename = "Unfiltered")]
    pub unfiltered: Option<bool>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "Security")]
    pub security: Option<String>,
    #[yaserde(rename = "GroupRange")]
    pub group_range: Vec<GroupRanget>,
    #[yaserde(rename = "GroupAddress")]
    pub group_address: Vec<GroupAddresst>,
}

// GroupRanges ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupRanges {
    #[yaserde(rename = "GroupRange")]
    pub group_range: Vec<GroupRanget>,
}

// GroupAddressest ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupAddressest {
    #[yaserde(rename = "GroupRanges")]
    pub group_ranges: GroupRanges,
}

// BusAccesst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BusAccesst {
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Edi")]
    pub edi: String,
    #[yaserde(rename = "Parameter")]
    pub parameter: String,
}

// GroupAddresst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupAddresst {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(rename = "Address")]
    pub address: u16,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Unfiltered")]
    pub unfiltered: Option<bool>,
    #[yaserde(rename = "Central")]
    pub central: Option<bool>,
    #[yaserde(rename = "Global")]
    pub global: Option<bool>,
    #[yaserde(rename = "DatapointType")]
    pub datapoint_type: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "Key")]
    pub key: Option<String>,
    #[yaserde(rename = "Security")]
    pub security: Option<String>,
}

// Locationst ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Locationst {
    #[yaserde(rename = "Space")]
    pub space: Vec<Spacet>,
}

// Spacet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Spacet {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Type")]
    pub type_attr: String,
    #[yaserde(rename = "Usage")]
    pub usage: Option<String>,
    #[yaserde(attribute, rename = "Number")]
    pub number: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "DefaultLine")]
    pub default_line: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "Space")]
    pub space: Vec<Spacet>,
    #[yaserde(rename = "DeviceInstanceRef")]
    pub device_instance_ref: Vec<DeviceInstanceReft>,
    #[yaserde(rename = "Function")]
    pub function: Vec<Functiont>,
}

// Functiont ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Functiont {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(rename = "Type")]
    pub type_attr: Option<String>,
    #[yaserde(rename = "Implements")]
    pub implements: Vec<String>,
    #[yaserde(attribute, rename = "Number")]
    pub number: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "DefaultGroupRange")]
    pub default_group_range: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "GroupAddressRef")]
    pub group_address_ref: Vec<GroupAddressReft>,
}

// GroupAddressReft ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct GroupAddressReft {
    #[yaserde(attribute, rename = "Id")]
    pub id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(rename = "Role")]
    pub role: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
}

// Tradest ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Tradest {
    #[yaserde(rename = "Trade")]
    pub trade: Vec<Tradet>,
}

// Tradet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Tradet {
    #[yaserde(attribute, rename = "Id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
    #[yaserde(attribute, rename = "Number")]
    pub number: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "CompletionStatus")]
    pub completion_status: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Puid")]
    pub puid: i32,
    #[yaserde(rename = "Trade")]
    pub trade: Vec<Tradet>,
    #[yaserde(rename = "DeviceInstanceRef")]
    pub device_instance_ref: Vec<DeviceInstanceReft>,
}

// ProjectTracet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ProjectTracet {
    #[yaserde(rename = "Date")]
    pub date: u16,
    #[yaserde(rename = "UserName")]
    pub user_name: String,
    #[yaserde(rename = "Comment")]
    pub comment: String,
}

// ToDoItemt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct ToDoItemt {
    #[yaserde(rename = "Description")]
    pub description: String,
    #[yaserde(rename = "ObjectPath")]
    pub object_path: Option<String>,
    #[yaserde(rename = "Status")]
    pub status: String,
}

// UserFilet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct UserFilet {
    #[yaserde(rename = "Filename")]
    pub filename: String,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
}

// AddinDatat ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct AddinDatat {
    #[yaserde(rename = "AddinId")]
    pub addin_id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
}

// DeviceCertificatet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct DeviceCertificatet {
    #[yaserde(rename = "SerialNumber")]
    pub serial_number: String,
    #[yaserde(rename = "FDSK")]
    pub fdsk: String,
}

// IPConfigt ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct IPConfigt {
    #[yaserde(rename = "Assign")]
    pub assign: Option<String>,
    #[yaserde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[yaserde(rename = "SubnetMask")]
    pub subnet_mask: Option<String>,
    #[yaserde(rename = "DefaultGateway")]
    pub default_gateway: Option<String>,
    #[yaserde(rename = "MACAddress")]
    pub mac_address: Option<String>,
}

// Connector ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct Connector {
    #[yaserde(rename = "GroupAddressRefId")]
    pub group_address_ref_id: String,
}

// BusInterfacet ...
#[derive(Debug, Default, Clone, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(namespace = "http://knx.org/xml/project/20")]
pub struct BusInterfacet {
    #[yaserde(attribute, rename = "RefId")]
    pub ref_id: String,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(rename = "Description")]
    pub description: Option<String>,
    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,
    #[yaserde(rename = "Password")]
    pub password: Option<String>,
    #[yaserde(rename = "PasswordHash")]
    pub password_hash: Option<String>,
    #[yaserde(rename = "Connectors")]
    pub connectors: Connectors,
}
