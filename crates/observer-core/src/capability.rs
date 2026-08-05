use serde::{Deserialize, Serialize};

/// Granular capability — what an agent is allowed to do.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilitySpec {
    pub action: String,
    pub resource: String,
    pub maximum_amount: Option<f64>,
    pub duration: CapabilityDuration,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityDuration {
    OneTime,
    Minutes(u64),
    Hours(u64),
    Session,
    UntilRevoked,
}

/// Core capability classes for the capability model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClass {
    Calendar { operation: CalendarOperation },
    Email { operation: EmailOperation },
    Contacts { operation: ContactOperation },
    Files { operation: FileOperation },
    Photos { operation: PhotoOperation },
    Location { precision: LocationPrecision },
    Browser { operation: BrowserOperation },
    Network { operation: NetworkOperation },
    Payment { operation: PaymentOperation },
    Identity { operation: IdentityOperation },
    Credentials { operation: CredentialOperation },
    Process { operation: ProcessOperation },
    Device { resource: DeviceResource },
    Model { operation: ModelOperation },
    Home { operation: HomeOperation },
    Robot { operation: RobotOperation },
    User { operation: UserOperation },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CalendarOperation { Read, Create, Update, Delete }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EmailOperation { Read, Draft, Send, Delete }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContactOperation { Read, Share }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileOperation { Read, Create, Write, Move, Delete, Share }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PhotoOperation { Read, Share }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LocationPrecision { Approximate, Precise }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserOperation { Navigate, FormFill, Download, Upload }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkOperation { Connect, Listen }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentOperation { Quote, Purchase, Transfer }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IdentityOperation { Read, Share }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialOperation { Request }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProcessOperation { Spawn, Stop }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceResource { Camera, Microphone, Bluetooth, Usb, Gpu }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModelOperation { Invoke }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HomeOperation { Unlock, Lock, CameraView, DeviceControl }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RobotOperation { Move, Manipulate, CaptureMedia }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserOperation { Notify, Prompt }
