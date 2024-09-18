// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -D Default -D PartialEq -A -d -f -
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::api::core::v1::ObjectReference;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;

/// MachineSpec defines the desired state of Machine.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
#[kube(
    group = "cluster.x-k8s.io",
    version = "v1beta1",
    kind = "Machine",
    plural = "machines"
)]
#[kube(namespaced)]
#[kube(status = "MachineStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct MachineSpec {
    /// Bootstrap is a reference to a local struct which encapsulates
    /// fields to configure the Machine’s bootstrapping mechanism.
    pub bootstrap: MachineBootstrap,
    /// ClusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// FailureDomain is the failure domain the machine will be created in.
    /// Must match a key in the FailureDomains map stored on the cluster object.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureDomain"
    )]
    pub failure_domain: Option<String>,
    /// InfrastructureRef is a required reference to a custom resource
    /// offered by an infrastructure provider.
    #[serde(rename = "infrastructureRef")]
    pub infrastructure_ref: ObjectReference,
    /// NodeDeletionTimeout defines how long the controller will attempt to delete the Node that the Machine
    /// hosts after the Machine is marked for deletion. A duration of 0 will retry deletion indefinitely.
    /// Defaults to 10 seconds.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeDeletionTimeout"
    )]
    pub node_deletion_timeout: Option<String>,
    /// NodeDrainTimeout is the total amount of time that the controller will spend on draining a node.
    /// The default value is 0, meaning that the node can be drained without any time limitations.
    /// NOTE: NodeDrainTimeout is different from `kubectl drain --timeout`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeDrainTimeout"
    )]
    pub node_drain_timeout: Option<String>,
    /// NodeVolumeDetachTimeout is the total amount of time that the controller will spend on waiting for all volumes
    /// to be detached. The default value is 0, meaning that the volumes can be detached without any time limitations.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeVolumeDetachTimeout"
    )]
    pub node_volume_detach_timeout: Option<String>,
    /// ProviderID is the identification ID of the machine provided by the provider.
    /// This field must match the provider ID as seen on the node object corresponding to this machine.
    /// This field is required by higher level consumers of cluster-api. Example use case is cluster autoscaler
    /// with cluster-api as provider. Clean-up logic in the autoscaler compares machines to nodes to find out
    /// machines at provider which could not get registered as Kubernetes nodes. With cluster-api as a
    /// generic out-of-tree provider for autoscaler, this field is required by autoscaler to be
    /// able to have a provider view of the list of machines. Another list of nodes is queried from the k8s apiserver
    /// and then a comparison is done to find out unregistered machines and are marked for delete.
    /// This field will be set by the actuators and consumed by higher level entities like autoscaler that will
    /// be interfacing with cluster-api as generic provider.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "providerID"
    )]
    pub provider_id: Option<String>,
    /// Version defines the desired Kubernetes version.
    /// This field is meant to be optionally used by bootstrap providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Bootstrap is a reference to a local struct which encapsulates
/// fields to configure the Machine’s bootstrapping mechanism.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineBootstrap {
    /// ConfigRef is a reference to a bootstrap provider-specific resource
    /// that holds configuration details. The reference is optional to
    /// allow users/operators to specify Bootstrap.DataSecretName without
    /// the need of a controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<ObjectReference>,
    /// DataSecretName is the name of the secret that stores the bootstrap data script.
    /// If nil, the Machine should remain in the Pending state.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSecretName"
    )]
    pub data_secret_name: Option<String>,
}

/// ConfigRef is a reference to a bootstrap provider-specific resource
/// that holds configuration details. The reference is optional to
/// allow users/operators to specify Bootstrap.DataSecretName without
/// the need of a controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineBootstrapConfigRef {
    /// API version of the referent.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// InfrastructureRef is a required reference to a custom resource
/// offered by an infrastructure provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineInfrastructureRef {
    /// API version of the referent.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// MachineStatus defines the observed state of Machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineStatus {
    /// Addresses is a list of addresses assigned to the machine.
    /// This field is copied from the infrastructure provider reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<MachineStatusAddresses>>,
    /// BootstrapReady is the state of the bootstrap provider.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bootstrapReady"
    )]
    pub bootstrap_ready: Option<bool>,
    /// CertificatesExpiryDate is the expiry date of the machine certificates.
    /// This value is only set for control plane machines.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificatesExpiryDate"
    )]
    pub certificates_expiry_date: Option<String>,
    /// Conditions defines current service state of the Machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureMessage will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a more verbose string suitable
    /// for logging and human consumption.
    ///
    ///
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the Machine's spec or the configuration of
    /// the controller, and that manual intervention is required. Examples
    /// of terminal errors would be invalid combinations of settings in the
    /// spec, values that are unsupported by the controller, or the
    /// responsible controller itself being critically misconfigured.
    ///
    ///
    /// Any transient errors that occur during the reconciliation of Machines
    /// can be added as events to the Machine object and/or logged in the
    /// controller's output.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureMessage"
    )]
    pub failure_message: Option<String>,
    /// FailureReason will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a succinct value suitable
    /// for machine interpretation.
    ///
    ///
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the Machine's spec or the configuration of
    /// the controller, and that manual intervention is required. Examples
    /// of terminal errors would be invalid combinations of settings in the
    /// spec, values that are unsupported by the controller, or the
    /// responsible controller itself being critically misconfigured.
    ///
    ///
    /// Any transient errors that occur during the reconciliation of Machines
    /// can be added as events to the Machine object and/or logged in the
    /// controller's output.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureReason"
    )]
    pub failure_reason: Option<String>,
    /// InfrastructureReady is the state of the infrastructure provider.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "infrastructureReady"
    )]
    pub infrastructure_ready: Option<bool>,
    /// LastUpdated identifies when the phase of the Machine last transitioned.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<String>,
    /// NodeInfo is a set of ids/uuids to uniquely identify the node.
    /// More info: https://kubernetes.io/docs/concepts/nodes/node/#info
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeInfo")]
    pub node_info: Option<MachineStatusNodeInfo>,
    /// NodeRef will point to the corresponding Node if it exists.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeRef")]
    pub node_ref: Option<ObjectReference>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "observedGeneration"
    )]
    pub observed_generation: Option<i64>,
    /// Phase represents the current phase of machine actuation.
    /// E.g. Pending, Running, Terminating, Failed etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// MachineAddress contains information for the node's address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineStatusAddresses {
    /// The machine address.
    pub address: String,
    /// Machine address type, one of Hostname, ExternalIP, InternalIP, ExternalDNS or InternalDNS.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// NodeInfo is a set of ids/uuids to uniquely identify the node.
/// More info: https://kubernetes.io/docs/concepts/nodes/node/#info
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineStatusNodeInfo {
    /// The Architecture reported by the node
    pub architecture: String,
    /// Boot ID reported by the node.
    #[serde(rename = "bootID")]
    pub boot_id: String,
    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
    #[serde(rename = "containerRuntimeVersion")]
    pub container_runtime_version: String,
    /// Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,
    /// KubeProxy Version reported by the node.
    #[serde(rename = "kubeProxyVersion")]
    pub kube_proxy_version: String,
    /// Kubelet Version reported by the node.
    #[serde(rename = "kubeletVersion")]
    pub kubelet_version: String,
    /// MachineID reported by the node. For unique machine identification
    /// in the cluster this field is preferred. Learn more from man(5)
    /// machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
    #[serde(rename = "machineID")]
    pub machine_id: String,
    /// The Operating System reported by the node
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    /// OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
    #[serde(rename = "osImage")]
    pub os_image: String,
    /// SystemUUID reported by the node. For unique machine identification
    /// MachineID is preferred. This field is specific to Red Hat hosts
    /// https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid
    #[serde(rename = "systemUUID")]
    pub system_uuid: String,
}

/// NodeRef will point to the corresponding Node if it exists.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct MachineStatusNodeRef {
    /// API version of the referent.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
