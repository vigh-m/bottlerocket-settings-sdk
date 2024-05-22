use model_derive::model;
use std::collections::HashMap;

use crate::{
    BootSettings, BootstrapContainer, CloudFormationSettings, DnsSettings, HostContainer,
    NetworkSettings,
};
use modeled_types::Identifier;

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[model(rename = "settings", impl_default = true)]
struct Settings {
    motd: settings_extension_motd::MotdV1,
    updates: settings_extension_updates::UpdatesSettingsV1,
    host_containers: HashMap<Identifier, HostContainer>,
    bootstrap_containers: HashMap<Identifier, BootstrapContainer>,
    ntp: settings_extension_ntp::NtpSettingsV1,
    network: NetworkSettings,
    kernel: settings_extension_kernel::KernelSettingsV1,
    boot: BootSettings,
    aws: settings_extension_aws::AwsSettingsV1,
    metrics: settings_extension_metrics::MetricsSettingsV1,
    pki: settings_extension_pki::PkiSettingsV1,
    container_registry: settings_extension_container_registry::RegistrySettingsV1,
    oci_hooks: settings_extension_oci_hooks::OciHooksSettingsV1,
    cloudformation: CloudFormationSettings,
    dns: DnsSettings,
}
