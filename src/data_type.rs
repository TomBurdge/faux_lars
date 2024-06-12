use crate::expressions::Kwargs;
use fake::faker::address::raw::*;
use fake::faker::administrative::raw::*;
use fake::faker::automotive::raw::*;
use fake::faker::barcode::raw::*;
use fake::faker::company::raw::*;
use fake::faker::creditcard::raw::*;
use fake::faker::currency::raw::*;
use fake::faker::filesystem::raw::*;
use fake::faker::finance::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use polars::prelude::*;
use std::string::String;
// use fake::faker::http::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::job::raw::{Position, Seniority};
use fake::faker::lorem::raw::Word;
use fake::faker::phone_number::raw::*;

#[derive(Debug)]
pub enum DataType {
    EnglishName { length: usize },
    FrenchName { length: usize },
    JapaneseName { length: usize },
    PortgueseName { length: usize },
    ArabicName { length: usize },
    TraditionalChineseName { length: usize },
    SimplifiedChineseName { length: usize },
    EnglishFirstName { length: usize },
    FrenchFirstName { length: usize },
    JapaneseFirstName { length: usize },
    PortgueseFirstName { length: usize },
    ArabicFirstName { length: usize },
    TraditionalChineseFirstName { length: usize },
    SimplifiedChineseFirstName { length: usize },
    EnglishLastName { length: usize },
    FrenchLastName { length: usize },
    JapaneseLastName { length: usize },
    PortgueseLastName { length: usize },
    ArabicLastName { length: usize },
    TraditionalChineseLastName { length: usize },
    SimplifiedChineseLastName { length: usize },
    BuildingNumber { length: usize },
    EnglishCityName { length: usize },
    CountryCode { length: usize },
    EnglishCountryName { length: usize },
    Latitude { length: usize },
    Longitude { length: usize },
    PostCode { length: usize },
    SecondaryAddress { length: usize },
    SecondaryAddressType { length: usize },
    StateAbbr { length: usize },
    EnglishStateName { length: usize },
    BrPortugeseStateName { length: usize },
    EnglishStreetName { length: usize },
    BrPortugeseStreetName { length: usize },
    TimeZone { length: usize },
    ZipCode { length: usize },
    Isbn { length: usize },
    Isbn10 { length: usize },
    Isbn13 { length: usize },
    Bs { length: usize },
    EnglishCompanyName { length: usize },
    FrenchCompanyName { length: usize },
    JapaneseCompanyName { length: usize },
    PortugeseCompanyName { length: usize },
    ArabicCompanyName { length: usize },
    TraditionalChineseCompanyName { length: usize },
    SimplifiedChineseCompanyName { length: usize },
    Industry { length: usize },
    Profession { length: usize },
    CreditCardNumber { length: usize },
    CurrencyCode { length: usize },
    CurrencyName { length: usize },
    CurrencySymbol { length: usize },
    FrenchLicencePlate { length: usize },
    FrenchHealthInsuranceCode { length: usize },
    // in barcode:
    DirPath { length: usize },
    FileExtension { length: usize },
    FileName { length: usize },
    FilePath { length: usize },
    MimeType { length: usize },
    Semver { length: usize },
    SemverStable { length: usize },
    SemverUnstable { length: usize },
    Bic { length: usize },
    Isin { length: usize },
    EnglishFreeEmail { length: usize },
    FrenchFreeEmail { length: usize },
    PortgueseFreeEmail { length: usize },
    IP { length: usize },
    IPv4 { length: usize },
    IPv6 { length: usize },
    MACAddress { length: usize },
    Password { length: usize },
    EnglishSafeEmail { length: usize },
    FrenchSafeEmail { length: usize },
    PortugeseSafeEmail { length: usize },
    JapaneseSafeEmail { length: usize },
    ArabicSafeEmail { length: usize },
    TraditionalChineseSafeEmail { length: usize },
    SimplifiedChineseSafeEmail { length: usize },
    UserAgent { length: usize },
    UserName { length: usize },
    EnglishField { length: usize },
    JapaneseField { length: usize },
    SimplifiedChineseField { length: usize },
    EnglishPosition { length: usize },
    JapanesePosition { length: usize },
    SimplifiedChinesePosition { length: usize },
    EnglishSeniority { length: usize },
    JapaneseSeniority { length: usize },
    SimplifiedSeniority { length: usize },
    EnglishTitle { length: usize },
    JapaneseTitle { length: usize },
    SimplifiedChineseTitle { length: usize },
    Lorem { length: usize },
    CellNumber { length: usize },
    PhoneNumber { length: usize },
}

pub trait FromKwargs {
    fn from_kwargs(args: Kwargs) -> Result<Self, PolarsError>
    where
        Self: Sized;
}

impl FromKwargs for DataType {
    fn from_kwargs(args: Kwargs) -> Result<Self, PolarsError> {
        let data_type = args.data_type.as_str();
        let lang = args.lang.as_str();
        let length = args.length;

        match (data_type, lang) {
            ("name", "en") => Ok(DataType::EnglishName { length }),
            ("name", "fr") => Ok(DataType::FrenchName { length }),
            ("name", "jp") => Ok(DataType::JapaneseName { length }),
            ("name", "ar") => Ok(DataType::ArabicName { length }),
            ("name", "pt_br") => Ok(DataType::PortgueseName { length }),
            ("name", "zh_tw") => Ok(DataType::TraditionalChineseName { length }),
            ("name", "zh_cn") => Ok(DataType::SimplifiedChineseName { length }),
            ("first_name", "en") => Ok(DataType::EnglishFirstName { length }),
            ("first_name", "fr") => Ok(DataType::FrenchFirstName { length }),
            ("first_name", "jp") => Ok(DataType::JapaneseFirstName { length }),
            ("first_name", "ar") => Ok(DataType::ArabicFirstName { length }),
            ("first_name", "pt_br") => Ok(DataType::PortgueseFirstName { length }),
            ("first_name", "zh_tw") => Ok(DataType::TraditionalChineseFirstName { length }),
            ("first_name", "zh_cn") => Ok(DataType::SimplifiedChineseFirstName { length }),
            ("last_name", "en") => Ok(DataType::EnglishLastName { length }),
            ("last_name", "fr") => Ok(DataType::FrenchLastName { length }),
            ("last_name", "jp") => Ok(DataType::JapaneseLastName { length }),
            ("last_name", "ar") => Ok(DataType::ArabicLastName { length }),
            ("last_name", "pt_br") => Ok(DataType::PortgueseLastName { length }),
            ("last_name", "zh_tw") => Ok(DataType::TraditionalChineseLastName { length }),
            ("last_name", "zh_cn") => Ok(DataType::SimplifiedChineseLastName { length }),
            ("building_number", _) => Ok(DataType::BuildingNumber { length }),
            ("city_name", "en") => Ok(DataType::EnglishCityName { length }),
            ("country_code", _) => Ok(DataType::CountryCode { length }),
            ("country_name", "en") => Ok(DataType::EnglishCountryName { length }),
            ("latitude", "en") => Ok(DataType::Latitude { length }),
            ("longitude", "en") => Ok(DataType::Longitude { length }),
            ("postcode", "en") => Ok(DataType::PostCode { length }),
            ("secondary_address", "en") => Ok(DataType::SecondaryAddress { length }),
            ("secondary_address_type", "en") => Ok(DataType::SecondaryAddressType { length }),
            ("state_abbr", "en") => Ok(DataType::StateAbbr { length }),
            ("state_name", "pt_br") => Ok(DataType::BrPortugeseStateName { length }),
            ("state_name", "en") => Ok(DataType::EnglishStateName { length }),
            ("street_name", "pt_br") => Ok(DataType::BrPortugeseStreetName { length }),
            ("street_name", "en") => Ok(DataType::EnglishStreetName { length }),
            ("time_zone", "en") => Ok(DataType::TimeZone { length }),
            ("zip_code", "en") => Ok(DataType::ZipCode { length }),
            ("isbn", _) => Ok(DataType::Isbn { length }),
            ("isbn_10", _) => Ok(DataType::Isbn10 { length }),
            ("isbn_13", _) => Ok(DataType::Isbn13 { length }),
            ("bs", "en") => Ok(DataType::Bs { length }),
            ("company_name", "en") => Ok(DataType::EnglishCompanyName { length }),
            ("company_name", "fr") => Ok(DataType::FrenchCompanyName { length }),
            ("company_name", "jp") => Ok(DataType::JapaneseCompanyName { length }),
            ("company_name", "pt_br") => Ok(DataType::PortugeseCompanyName { length }),
            ("company_name", "ar") => Ok(DataType::ArabicCompanyName { length }),
            ("company_name", "zh_tw") => Ok(DataType::TraditionalChineseCompanyName { length }),
            ("company_name", "zh_cn") => Ok(DataType::SimplifiedChineseCompanyName { length }),
            ("industry", "en") => Ok(DataType::Industry { length }),
            ("profession", "en") => Ok(DataType::Profession { length }),
            ("credit_card_number", _) => Ok(DataType::CreditCardNumber { length }),
            ("currency_code", _) => Ok(DataType::CurrencyCode { length }),
            ("currency_name", "en") => Ok(DataType::CurrencyName { length }),
            ("currency_symbol", _) => Ok(DataType::CurrencySymbol { length }),
            ("dir_path", "en") => Ok(DataType::DirPath { length }),
            ("file_ext", "en") => Ok(DataType::FileExtension { length }),
            ("file_name", "en") => Ok(DataType::FileName { length }),
            ("file_path", "en") => Ok(DataType::FilePath { length }),
            ("mime_type", "en") => Ok(DataType::MimeType { length }),
            ("semver", "en") => Ok(DataType::Semver { length }),
            ("semver_stable", "en") => Ok(DataType::SemverStable { length }),
            ("semver_unstable", "en") => Ok(DataType::SemverUnstable { length }),
            ("licence_plate", "fr") => Ok(DataType::FrenchLicencePlate { length }),
            ("health_insurance_code", "fr") => Ok(DataType::FrenchHealthInsuranceCode { length }),
            ("free_email", "en") => Ok(DataType::EnglishFreeEmail { length }),
            ("free_email", "fr") => Ok(DataType::FrenchFreeEmail { length }),
            ("free_email", "pt_br") => Ok(DataType::PortgueseFreeEmail { length }),
            ("bic", _) => Ok(DataType::Bic { length }),
            ("isin", _) => Ok(DataType::Isin { length }),
            ("ip", _) => Ok(DataType::IP { length }),
            ("ipv4", _) => Ok(DataType::IPv4 { length }),
            ("ipv6", _) => Ok(DataType::IPv6 { length }),
            ("mac_address", _) => Ok(DataType::MACAddress { length }),
            // password with Latin script
            ("password", "en" | "fr" | "pt_br") => Ok(DataType::Password { length }),
            ("safe_email", "en") => Ok(DataType::EnglishSafeEmail { length }),
            ("safe_email", "fr") => Ok(DataType::FrenchSafeEmail { length }),
            ("safe_email", "pt_br") => Ok(DataType::PortugeseSafeEmail { length }),
            ("safe_email", "jp") => Ok(DataType::JapaneseSafeEmail { length }),
            ("safe_email", "ar") => Ok(DataType::ArabicSafeEmail { length }),
            ("safe_email", "zh_tw") => Ok(DataType::TraditionalChineseSafeEmail { length }),
            ("safe_email", "zh_cn") => Ok(DataType::SimplifiedChineseSafeEmail { length }),
            ("user_agent", _) => Ok(DataType::UserAgent { length }),
            ("user_name", _) => Ok(DataType::UserName { length }),
            ("field", "en") => Ok(DataType::EnglishField { length }),
            ("field", "jp") => Ok(DataType::JapaneseField { length }),
            ("field", "zh_cn") => Ok(DataType::SimplifiedChineseField { length }),
            ("position", "en") => Ok(DataType::EnglishPosition { length }),
            ("position", "jp") => Ok(DataType::JapanesePosition { length }),
            ("position", "zh_cn") => Ok(DataType::SimplifiedChinesePosition { length }),
            ("seniority", "en") => Ok(DataType::EnglishSeniority { length }),
            ("seniority", "jp") => Ok(DataType::JapaneseSeniority { length }),
            ("seniority", "zh_cn") => Ok(DataType::SimplifiedSeniority { length }),
            ("title", "en") => Ok(DataType::EnglishTitle { length }),
            ("title", "jp") => Ok(DataType::JapaneseTitle { length }),
            ("title", "zh_cn") => Ok(DataType::SimplifiedChineseTitle { length }),
            ("lorem", _) => Ok(DataType::Lorem { length }),
            ("cell_number" | "mobile_number", _) => Ok(DataType::CellNumber { length }),
            ("phone_number", _) => Ok(DataType::PhoneNumber { length }),
            _ => Err(PolarsError::ComputeError(
                format!(
                    "Unsupported language & data type combination. Data type: {} and language: {}",
                    data_type, lang
                )
                .into(),
            )),
        }
    }
}

impl DataType {
    pub fn generate(&self) -> polars::prelude::Series {
        match self {
            DataType::EnglishName { length } => {
                Series::new("out", fake::vec![String as Name(EN); *length])
            }
            DataType::FrenchName { length } => {
                Series::new("out", fake::vec![String as Name(FR_FR); *length])
            }
            DataType::JapaneseName { length } => {
                Series::new("out", fake::vec![String as Name(JA_JP); *length])
            }
            DataType::PortgueseName { length } => {
                Series::new("out", fake::vec![String as Name(PT_BR); *length])
            }
            DataType::ArabicName { length } => {
                Series::new("out", fake::vec![String as Name(AR_SA); *length])
            }
            DataType::TraditionalChineseName { length } => {
                Series::new("out", fake::vec![String as Name(ZH_TW); *length])
            }
            DataType::SimplifiedChineseName { length } => {
                Series::new("out", fake::vec![String as Name(ZH_TW); *length])
            }
            DataType::EnglishFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(EN); *length])
            }
            DataType::FrenchFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(FR_FR); *length])
            }
            DataType::JapaneseFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(JA_JP); *length])
            }
            DataType::PortgueseFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(PT_BR); *length])
            }
            DataType::ArabicFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(AR_SA); *length])
            }
            DataType::TraditionalChineseFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(ZH_TW); *length])
            }
            DataType::SimplifiedChineseFirstName { length } => {
                Series::new("out", fake::vec![String as FirstName(ZH_TW); *length])
            }
            DataType::EnglishLastName { length } => {
                Series::new("out", fake::vec![String as LastName(EN); *length])
            }
            DataType::FrenchLastName { length } => {
                Series::new("out", fake::vec![String as LastName(FR_FR); *length])
            }
            DataType::JapaneseLastName { length } => {
                Series::new("out", fake::vec![String as LastName(JA_JP); *length])
            }
            DataType::PortgueseLastName { length } => {
                Series::new("out", fake::vec![String as LastName(PT_BR); *length])
            }
            DataType::ArabicLastName { length } => {
                Series::new("out", fake::vec![String as LastName(AR_SA); *length])
            }
            DataType::TraditionalChineseLastName { length } => {
                Series::new("out", fake::vec![String as LastName(ZH_TW); *length])
            }
            DataType::SimplifiedChineseLastName { length } => {
                Series::new("out", fake::vec![String as LastName(ZH_TW); *length])
            }
            DataType::EnglishCityName { length } => {
                Series::new("out", fake::vec![String as CityName(EN); *length])
            }
            // TODO : return to the data types for the non-str instances
            // might need to not use the macro
            DataType::BuildingNumber { length } => {
                Series::new("out", fake::vec![String as BuildingNumber(EN); *length])
            }
            DataType::Latitude { length } => {
                Series::new("out", fake::vec![String as Latitude(EN); *length])
            }
            DataType::Longitude { length } => {
                Series::new("out", fake::vec![String as Longitude(EN); *length])
            }
            DataType::CreditCardNumber { length } => Series::new(
                "credit_card_number",
                fake::vec![String as CreditCardNumber(EN); *length],
            ),
            DataType::ZipCode { length } => {
                Series::new("time_zone", fake::vec![String as ZipCode(EN); *length])
            }
            DataType::CellNumber { length } => {
                Series::new("lorem", fake::vec![String as CellNumber(EN); *length])
            }
            DataType::PhoneNumber { length } => {
                Series::new("lorem", fake::vec![String as PhoneNumber(EN); *length])
            }
            // might be wrong type
            DataType::FrenchLicencePlate { length } => Series::new(
                "licence_plate",
                fake::vec![String as LicencePlate(FR_FR); *length],
            ),
            DataType::FrenchHealthInsuranceCode { length } => Series::new(
                "health_insurance_code",
                fake::vec![String as HealthInsuranceCode(FR_FR); *length],
            ),
            DataType::Bic { length } => Series::new("bic", fake::vec![String as Bic(EN); *length]),
            DataType::Isin { length } => {
                Series::new("isin", fake::vec![String as Isin(EN); *length])
            }
            // ISO country code doesn't vary by country
            DataType::CountryCode { length } => {
                Series::new("out", fake::vec![String as CountryCode(EN); *length])
            }
            DataType::Isbn { length } => {
                Series::new("isbn", fake::vec![String as Isbn(EN); *length])
            }
            DataType::Isbn10 { length } => {
                Series::new("isbn_10", fake::vec![String as Isbn10(EN); *length])
            }
            DataType::Isbn13 { length } => {
                Series::new("isbn_13", fake::vec![String as Isbn13(EN); *length])
            }
            DataType::CurrencyCode { length } => Series::new(
                "currency_code",
                fake::vec![String as CurrencyCode(EN); *length],
            ),
            DataType::CurrencySymbol { length } => Series::new(
                "currency_symbol",
                fake::vec![String as CurrencySymbol(EN); *length],
            ),
            // back to usual
            DataType::EnglishCountryName { length } => Series::new(
                "country_name",
                fake::vec![String as CountryName(EN); *length],
            ),
            DataType::PostCode { length } => {
                Series::new("post_code", fake::vec![String as PostCode(EN); *length])
            }
            DataType::SecondaryAddress { length } => Series::new(
                "secondary_address",
                fake::vec![String as SecondaryAddress(EN); *length],
            ),
            DataType::SecondaryAddressType { length } => Series::new(
                "secondary_address_type",
                fake::vec![String as SecondaryAddressType(EN); *length],
            ),
            DataType::StateAbbr { length } => Series::new(
                "state_abbreviation",
                fake::vec![String as StateAbbr(EN); *length],
            ),
            DataType::EnglishStateName { length } => {
                Series::new("state_name", fake::vec![String as StateName(EN); *length])
            }
            DataType::BrPortugeseStateName { length } => Series::new(
                "state_name",
                fake::vec![String as StateName(PT_BR); *length],
            ),
            DataType::EnglishStreetName { length } => {
                Series::new("street", fake::vec![String as StreetName(EN); *length])
            }
            DataType::BrPortugeseStreetName { length } => {
                Series::new("street", fake::vec![String as StreetName(PT_BR); *length])
            }
            DataType::TimeZone { length } => {
                Series::new("time_zone", fake::vec![String as TimeZone(EN); *length])
            }
            DataType::Bs { length } => Series::new("bs", fake::vec![String as Bs(EN); *length]),
            DataType::EnglishCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(EN); *length],
            ),
            DataType::FrenchCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(FR_FR); *length],
            ),
            DataType::TraditionalChineseCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(ZH_TW); *length],
            ),
            DataType::SimplifiedChineseCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(ZH_CN); *length],
            ),
            DataType::JapaneseCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(JA_JP); *length],
            ),
            DataType::PortugeseCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(PT_BR); *length],
            ),
            DataType::ArabicCompanyName { length } => Series::new(
                "company_name",
                fake::vec![String as CompanyName(AR_SA); *length],
            ),
            DataType::Industry { length } => {
                Series::new("industry", fake::vec![String as Industry(EN); *length])
            }
            DataType::Profession { length } => {
                Series::new("profession", fake::vec![String as Profession(EN); *length])
            }
            DataType::CurrencyName { length } => {
                Series::new("currency", fake::vec![String as CurrencyName(EN); *length])
            }
            DataType::DirPath { length } => {
                Series::new("dir_path", fake::vec![String as DirPath(EN); *length])
            }
            DataType::FileExtension { length } => {
                Series::new("file_ext", fake::vec![String as FileExtension(EN); *length])
            }
            DataType::FileName { length } => {
                Series::new("file_name", fake::vec![String as FileName(EN); *length])
            }
            DataType::FilePath { length } => {
                Series::new("file_path", fake::vec![String as FilePath(EN); *length])
            }
            DataType::MimeType { length } => {
                Series::new("mime_type", fake::vec![String as MimeType(EN); *length])
            }
            DataType::Semver { length } => {
                Series::new("semver", fake::vec![String as Semver(EN); *length])
            }
            DataType::SemverStable { length } => Series::new(
                "semver_stable",
                fake::vec![String as SemverStable(EN); *length],
            ),
            DataType::SemverUnstable { length } => Series::new(
                "semver_unstable",
                fake::vec![String as SemverUnstable(EN); *length],
            ),
            DataType::FrenchFreeEmail { length } => Series::new(
                "free_email",
                fake::vec![String as FreeEmail(FR_FR); *length],
            ),
            DataType::EnglishFreeEmail { length } => {
                Series::new("free_email", fake::vec![String as FreeEmail(EN); *length])
            }
            DataType::PortgueseFreeEmail { length } => Series::new(
                "free_email",
                fake::vec![String as FreeEmail(PT_BR); *length],
            ),
            DataType::IP { length } => Series::new("ip", fake::vec![String as IP(EN); *length]),
            DataType::IPv4 { length } => {
                Series::new("ipv4", fake::vec![String as IPv4(EN); *length])
            }
            DataType::IPv6 { length } => {
                Series::new("ipv6", fake::vec![String as IPv6(EN); *length])
            }
            DataType::MACAddress { length } => {
                Series::new("mac_address", fake::vec![String as MACAddress(EN); *length])
            }
            DataType::Password { length } => Series::new(
                "password",
                fake::vec![String as Password(EN, 9..10); *length],
            ),
            DataType::EnglishSafeEmail { length } => {
                Series::new("safe_email", fake::vec![String as SafeEmail(EN); *length])
            }
            DataType::FrenchSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(FR_FR); *length],
            ),
            DataType::PortugeseSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(PT_BR); *length],
            ),
            DataType::JapaneseSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(JA_JP); *length],
            ),
            DataType::ArabicSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(AR_SA); *length],
            ),
            DataType::TraditionalChineseSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(ZH_TW); *length],
            ),
            DataType::SimplifiedChineseSafeEmail { length } => Series::new(
                "safe_email",
                fake::vec![String as SafeEmail(ZH_CN); *length],
            ),
            DataType::UserAgent { length } => {
                Series::new("user_agent", fake::vec![String as UserAgent(EN); *length])
            }
            DataType::UserName { length } => {
                Series::new("user_agent", fake::vec![String as Username(EN); *length])
            }
            DataType::EnglishField { length } => Series::new(
                "field",
                fake::vec![String as fake::faker::job::raw::Field(EN); *length],
            ),
            DataType::SimplifiedChineseField { length } => Series::new(
                "field",
                fake::vec![String as fake::faker::job::raw::Field(ZH_CN); *length],
            ),
            DataType::JapaneseField { length } => Series::new(
                "field",
                fake::vec![String as fake::faker::job::raw::Field(JA_JP); *length],
            ),
            DataType::EnglishPosition { length } => {
                Series::new("position", fake::vec![String as Position(EN); *length])
            }
            DataType::JapanesePosition { length } => {
                Series::new("position", fake::vec![String as Position(JA_JP); *length])
            }
            DataType::SimplifiedChinesePosition { length } => {
                Series::new("position", fake::vec![String as Position(ZH_CN); *length])
            }
            DataType::EnglishSeniority { length } => {
                Series::new("seniority", fake::vec![String as Seniority(EN); *length])
            }
            DataType::JapaneseSeniority { length } => {
                Series::new("seniority", fake::vec![String as Seniority(JA_JP); *length])
            }
            DataType::SimplifiedSeniority { length } => {
                Series::new("seniority", fake::vec![String as Position(ZH_CN); *length])
            }
            DataType::EnglishTitle { length } => Series::new(
                "title",
                fake::vec![String as fake::faker::job::raw::Title(EN); *length],
            ),
            DataType::JapaneseTitle { length } => Series::new(
                "title",
                fake::vec![String as fake::faker::job::raw::Title(JA_JP); *length],
            ),
            DataType::SimplifiedChineseTitle { length } => Series::new(
                "title",
                fake::vec![String as fake::faker::job::raw::Title(ZH_CN); *length],
            ),
            DataType::Lorem { length } => {
                Series::new("lorem", fake::vec![String as Word(ZH_CN); *length])
            }
        }
    }
}
