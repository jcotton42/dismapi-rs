#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use winapi::{ENUM, STRUCT};
use winapi::shared::basetsd::UINT64;
use winapi::shared::minwindef::*;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::winnt::{PCWSTR, VOID, HRESULT, PVOID, HANDLE};

pub type DismSession = UINT;

pub type DISM_PROGRESS_CALLBACK =
    Option<unsafe extern "system" fn(Current: UINT, Total: UINT, UserData: LPVOID)>;

pub const DISM_ONLINE_IMAGE: &str = "DISM_{53BFAE52-B167-4E2F-A258-0A37B57FF845}";

pub const DISM_SESSION_DEFAULT: DismSession = 0;

pub const DISM_MOUNT_READWRITE: DWORD = 0x00000000;
pub const DISM_MOUNT_READONLY: DWORD = 0x00000001;
pub const DISM_MOUNT_OPTIMIZE: DWORD = 0x00000002;
pub const DISM_MOUNT_CHECK_INTEGRITY: DWORD = 0x00000004;

pub const DISM_COMMIT_IMAGE: DWORD = 0x00000000;
pub const DISM_DISCARD_IMAGE: DWORD = 0x00000001;

pub const DISM_COMMIT_GENERATE_INTEGRITY: DWORD = 0x00010000;
pub const DISM_COMMIT_APPEND: DWORD = 0x00020000;

pub const DISM_COMMIT_MASK: DWORD = 0xffff0000;

ENUM!{enum DismLogLevel {
    DismLogErrors = 0,
    DismLogErrorsWarnings,
    DismLogErrorsWarningsInfo,
}}

ENUM!{enum DismImageIdentifier {
    DismImageIndex = 0,
    DismImageName,
}}

ENUM!{enum DismMountMode {
    DismReadWrite = 0,
    DismReadOnly,
}}

ENUM!{enum DismImageType {
    DismImageTypeUnsupported = -1i32 as u32,
    DismImageTypeWim = 0,
    DismImageTypeVhd = 1,
}}

ENUM!{enum DismImageBootable {
    DismImageBootableYes = 0,
    DismImageBootableNo,
    DismImageBootableUnknown,
}}

ENUM!{enum DismMountStatus {
    DismMountStatusOk = 0,
    DismMountStatusNeedsRemount,
    DismMountStatusInvalid,
}}

ENUM!{enum DismImageHealthState {
    DismImageHealthy = 0,
    DismImageRepairable,
    DismImageNonRepairable,
}}

ENUM!{enum DismPackageIdentifier {
    DismPackageNone = 0,
    DismPackageName,
    DismPackagePath,
}}

ENUM!{enum DismPackageFeatureState {
    DismStateNotPresent = 0,
    DismStateUninstallPending,
    DismStateStaged,
    DismStateResolved,
    DismStateRemoved = DismStateResolved,
    DismStateInstalled,
    DismStateInstallPending,
    DismStateSuperseded,
    DismStatePartiallyInstalled,
}}

ENUM!{enum DismReleaseType {
    DismReleaseTypeCriticalUpdate = 0,
    DismReleaseTypeDriver,
    DismReleaseTypeFeaturePack,
    DismReleaseTypeHotfix,
    DismReleaseTypeSecurityUpdate,
    DismReleaseTypeSoftwareUpdate,
    DismReleaseTypeUpdate,
    DismReleaseTypeUpdateRollup,
    DismReleaseTypeLanguagePack,
    DismReleaseTypeFoundation,
    DismReleaseTypeServicePack,
    DismReleaseTypeProduct,
    DismReleaseTypeLocalPack,
    DismReleaseTypeOther,
    DismReleaseTypeOnDemandPack,
}}

ENUM!{enum DismRestartType {
    DismRestartNo = 0,
    DismRestartPossible,
    DismRestartRequired,
}}

ENUM!{enum DismDriverSignature {
    DismDriverSignatureUnknown = 0,
    DismDriverSignatureUnsigned = 1,
    DismDriverSignatureSigned = 2,
}}

ENUM!{enum DismFullyOfflineInstallableType {
    DismFullyOfflineInstallable = 0,
    DismFullyOfflineNotInstallable,
    DismFullyOfflineInstallableUndetermined,
}}

STRUCT!{#[repr(packed)] struct DismPackage {
    PackageName: PCWSTR,
    PackageState: DismPackageFeatureState,
    ReleaseType: DismReleaseType,
    InstallTime: SYSTEMTIME,
}}

STRUCT!{#[repr(packed)] struct DismCustomProperty {
    Name: PCWSTR,
    Value: PCWSTR,
    Path: PCWSTR,
}}

STRUCT!{#[repr(packed)] struct DismFeature {
    FeatureName: PCWSTR,
    State: DismPackageFeatureState,
}}

STRUCT!{#[repr(packed)] struct DismCapability {
    Name: PCWSTR,
    State: DismPackageFeatureState,
}}

STRUCT!{#[repr(packed)] struct DismPackageInfo {
    PackageName: PCWSTR,
    PackageState: DismPackageFeatureState,
    ReleaseType: DismReleaseType,
    InstallTime: SYSTEMTIME,
    Applicable: BOOL,
    Copyright: PCWSTR,
    Company: PCWSTR,
    CreationTime: SYSTEMTIME,
    DisplayName: PCWSTR,
    Description: PCWSTR,
    InstallClient: PCWSTR,
    InstallPackageName: PCWSTR,
    LastUpdateTime: SYSTEMTIME,
    ProductName: PCWSTR,
    ProductVersion: PCWSTR,
    RestartRequired: DismRestartType,
    FullyOffline: DismFullyOfflineInstallableType,
    SupportInformation: PCWSTR,
    CustomProperty: *const DismCustomProperty,
    CustomPropertyCount: UINT,
    Feature: *const DismFeature,
    FeatureCount: UINT,
}}

STRUCT!{#[repr(packed)] struct DismPackageInfoEx {
    parent: DismPackageInfo,
    CapabilityId: PCWSTR,
}}

STRUCT!{#[repr(packed)] struct DismFeatureInfo {
    FeatureName: PCWSTR,
    FeatureState: DismPackageFeatureState,
    DisplayName: PCWSTR,
    Description: PCWSTR,
    RestartRequired: DismRestartType,
    CustomProperty: *const DismCustomProperty,
    CustomPropertyCount: UINT,
}}

STRUCT!{#[repr(packed)] struct DismCapabilityInfo {
    Name: PCWSTR,
    State: DismPackageFeatureState,
    DisplayName: PCWSTR,
    Description: PCWSTR,
    DownloadSize: DWORD,
    InstallSize: DWORD,
}}

STRUCT!{#[repr(packed)] struct DismString {
    Value: PCWSTR,
}}

pub type DismLanguage = DismString;

STRUCT!{#[repr(packed)] struct DismWimCustomizedInfo {
    Size: UINT,
    DirectoryCount: UINT,
    FileCount: UINT,
    CreatedTime: SYSTEMTIME,
    ModifiedTime: SYSTEMTIME,
}}

STRUCT!{#[repr(packed)] struct DismImageInfo {
    ImageType: DismImageType,
    ImageIndex: UINT,
    ImageName: PCWSTR,
    ImageDescription: PCWSTR,
    ImageSize: UINT64,
    Architecture: UINT,
    ProductName: PCWSTR,
    EditionId: PCWSTR,
    InstallationType: PCWSTR,
    Hal: PCWSTR,
    ProductType: PCWSTR,
    ProductSuite: PCWSTR,
    MajorVersion: UINT,
    MinorVersion: UINT,
    Build: UINT,
    SpBuild: UINT,
    SpLevel: UINT,
    Bootable: DismImageBootable,
    SystemRoot: PCWSTR,
    Language: *const DismLanguage,
    LanguageCount: UINT,
    DefaultLanguageIndex: UINT,
    CustomizedInfo: *const VOID,
}}

STRUCT!{#[repr(packed)] struct DismMountedImageInfo {
    MountPath: PCWSTR,
    ImageFilePath: PCWSTR,
    ImageIndex: UINT,
    MountMode: DismMountMode,
    MountStatus: DismMountStatus,
}}

STRUCT!{#[repr(packed)] struct DismDriverPackage {
    PublishedName: PCWSTR,
    OriginalFileName: PCWSTR,
    InBox: BOOL,
    CatalogFile: PCWSTR,
    ClassName: PCWSTR,
    ClassGuid: PCWSTR,
    ClassDescription: PCWSTR,
    BootCritical: BOOL,
    DriverSignature: DismDriverSignature,
    ProviderName: PCWSTR,
    Date: SYSTEMTIME,
    MajorVersion: UINT,
    MinorVersion: UINT,
    Build: UINT,
    Revision: UINT,
}}

STRUCT!{#[repr(packed)] struct DismDriver {
    ManufacturerName: PCWSTR,
    HardwareDescription: PCWSTR,
    HardwareId: PCWSTR,
    Architecture: UINT,
    ServiceName: PCWSTR,
    CompatibleIds: PCWSTR,
    ExcludeIds: PCWSTR,
}}

extern "system" {
    pub fn DismInitialize(
        LogLevel: DismLogLevel,
        LogFilePath: PCWSTR,
        ScratchDirectory: PCWSTR,
    ) -> HRESULT;

    pub fn DismShutdown() -> HRESULT;

    pub fn DismMountImage(
        ImageFilePath: PCWSTR,
        MountPath: PCWSTR,
        ImageIndex: UINT,
        ImageName: PCWSTR,
        ImageIdentifier: DismImageIdentifier,
        Flags: DWORD,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismUnmountImage(
        MountPath: PCWSTR,
        Flags: DWORD,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismOpenSession(
        ImagePath: PCWSTR,
        WindowsDirectory: PCWSTR,
        SystemDrive: PCWSTR,
        Session: *mut DismSession,
    ) -> HRESULT;

    pub fn DismCloseSession(Session: DismSession) -> HRESULT;

    pub fn DismGetLastErrorMessage(ErrorMessage: *mut *mut DismString) -> HRESULT;

    pub fn DismRemountImage(MountPath: PCWSTR) -> HRESULT;

    pub fn DismCommitImage(
        Session: DismSession,
        Flags: DWORD,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismGetImageInfo(
        ImageFilePath: PCWSTR,
        ImageInfo: *mut *mut DismImageInfo,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismGetMountedImageInfo(
        MountedImageInfo: *mut *mut DismMountedImageInfo,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismCleanupMountpoints() -> HRESULT;

    pub fn DismCheckImageHealth(
        Session: DismSession,
        ScanImage: BOOL,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
        ImageHealth: *mut DismImageHealthState,
    ) -> HRESULT;

    pub fn DismRestoreImageHealth(
        Session: DismSession,
        SourcePaths: *mut PCWSTR,
        SourcePathCount: UINT,
        LimitAccess: BOOL,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismDelete(DismStructure: *mut VOID) -> HRESULT;

    pub fn DismAddPackage(
        Session: DismSession,
        PackagePath: PCWSTR,
        IgnoreCheck: BOOL,
        PreventPending: BOOL,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismRemovePackage(
        Session: DismSession,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismEnableFeature(
        Session: DismSession,
        FeatureName: PCWSTR,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        LimitAccess: BOOL,
        SourcePaths: *mut PCWSTR,
        SourcePathCount: UINT,
        EnableAll: BOOL,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismDisableFeature(
        Session: DismSession,
        FeatureName: PCWSTR,
        PackageName: PCWSTR,
        RemovePayload: BOOL,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismGetPackages(
        Session: DismSession,
        Package: *mut *mut DismPackage,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismGetPackageInfo(
        Session: DismSession,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        PackageInfo: *mut *mut DismPackageInfo,
    ) -> HRESULT;

    pub fn DismGetPackageInfoEx(
        Session: DismSession,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        PackageInfoEx: *mut *mut DismPackageInfoEx,
    ) -> HRESULT;

    pub fn DismGetFeatures(
        Session: DismSession,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        Feature: *mut *mut DismFeature,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismGetFeatureInfo(
        Session: DismSession,
        FeatureName: PCWSTR,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        FeatureInfo: *mut *mut DismFeatureInfo,
    ) -> HRESULT;

    pub fn DismGetFeatureParent(
        Session: DismSession,
        FeatureName: PCWSTR,
        Identifier: PCWSTR,
        PackageIdentifier: DismPackageIdentifier,
        Feature: *mut *mut DismFeature,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismApplyUnattend(
        Session: DismSession,
        UnattendFile: PCWSTR,
        SingleSession: BOOL,
    ) -> HRESULT;

    pub fn DismAddDriver(
        Session: DismSession,
        DriverPath: PCWSTR,
        ForceUnsigned: BOOL,
    ) -> HRESULT;

    pub fn DismRemoveDriver(
        Session: DismSession,
        DriverPath: PCWSTR,
    ) -> HRESULT;

    pub fn DismGetDrivers(
        Session: DismSession,
        AllDrivers: BOOL,
        DriverPackage: *mut *mut DismDriverPackage,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismGetDriverInfo(
        Session: DismSession,
        DriverPath: PCWSTR,
        Driver: *mut *mut DismDriver,
        Count: *mut UINT,
        DriverPackage: *mut *mut DismDriverPackage,
    ) -> HRESULT;

    pub fn DismGetCapabilities(
        Session: DismSession,
        Capability: *mut *mut DismCapability,
        Count: *mut UINT,
    ) -> HRESULT;

    pub fn DismGetCapabilityInfo(
        Session: DismSession,
        Name: PCWSTR,
        Info: *mut *mut DismCapabilityInfo,
    ) -> HRESULT;

    pub fn DismAddCapability(
        Session: DismSession,
        Name: PCWSTR,
        LimitAccess: BOOL,
        SourcePaths: *mut PCWSTR,
        SourcePathCount: UINT,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;

    pub fn DismRemoveCapability(
        Session: DismSession,
        Name: PCWSTR,
        CancelEvent: HANDLE,
        Progress: DISM_PROGRESS_CALLBACK,
        UserData: PVOID,
    ) -> HRESULT;
}

pub const DISMAPI_E_DISMAPI_NOT_INITIALIZED: HRESULT = 0xC0040001u32 as HRESULT;
pub const DISMAPI_E_SHUTDOWN_IN_PROGRESS: HRESULT = 0xC0040002u32 as HRESULT;
pub const DISMAPI_E_OPEN_SESSION_HANDLES: HRESULT = 0xC0040003u32 as HRESULT;
pub const DISMAPI_E_INVALID_DISM_SESSION: HRESULT = 0xC0040004u32 as HRESULT;
pub const DISMAPI_E_INVALID_IMAGE_INDEX: HRESULT = 0xC0040005u32 as HRESULT;
pub const DISMAPI_E_INVALID_IMAGE_NAME: HRESULT = 0xC0040006u32 as HRESULT;
pub const DISMAPI_E_UNABLE_TO_UNMOUNT_IMAGE_PATH: HRESULT = 0xC0040007u32 as HRESULT;
pub const DISMAPI_E_LOGGING_DISABLED: HRESULT = 0xC0040009u32 as HRESULT;
pub const DISMAPI_E_OPEN_HANDLES_UNABLE_TO_UNMOUNT_IMAGE_PATH: HRESULT = 0xC004000Au32 as HRESULT;
pub const DISMAPI_E_OPEN_HANDLES_UNABLE_TO_MOUNT_IMAGE_PATH: HRESULT = 0xC004000Bu32 as HRESULT;
pub const DISMAPI_E_OPEN_HANDLES_UNABLE_TO_REMOUNT_IMAGE_PATH: HRESULT = 0xC004000Cu32 as HRESULT;
pub const DISMAPI_E_PARENT_FEATURE_DISABLED: HRESULT = 0xC004000Du32 as HRESULT;
pub const DISMAPI_E_MUST_SPECIFY_ONLINE_IMAGE: HRESULT = 0xC004000Eu32 as HRESULT;
pub const DISMAPI_E_INVALID_PRODUCT_KEY: HRESULT = 0xC004000Fu32 as HRESULT;
pub const DISMAPI_E_NEEDS_REMOUNT: HRESULT = 0xC1510114u32 as HRESULT;
pub const DISMAPI_E_UNKNOWN_FEATURE: HRESULT = 0x800f080cu32 as HRESULT;
pub const DISMAPI_E_BUSY: HRESULT = 0x800f0902u32 as HRESULT;
