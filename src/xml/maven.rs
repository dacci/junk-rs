use indexmap::IndexMap as Map;
use std::io::{Read, Write};
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use yaserde::de::Deserializer as YaDeserializer;
use yaserde::ser::Serializer as YaSerializer;
use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
    namespace = "http://maven.apache.org/POM/4.0.0",
    namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
    rename = "project"
)]
pub struct Model {
    #[yaserde(rename = "modelVersion")]
    pub model_version: Option<String>,

    pub parent: Option<Parent>,

    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
    pub packaging: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,

    #[yaserde(rename = "inceptionYear")]
    pub inception_year: Option<String>,

    pub organization: Option<Organization>,
    pub licenses: Option<Licenses>,
    pub developers: Option<Developers>,
    pub contributors: Option<Contributors>,

    #[yaserde(rename = "mailingLists")]
    pub mailing_lists: Option<MailingLists>,

    pub prerequisites: Option<Prerequisites>,
    pub modules: Option<Modules>,
    pub scm: Option<Scm>,

    #[yaserde(rename = "issueManagement")]
    pub issue_management: Option<IssueManagement>,

    #[yaserde(rename = "ciManagement")]
    pub ci_management: Option<CiManagement>,

    #[yaserde(rename = "distributionManagement")]
    pub distribution_management: Option<DistributionManagement>,

    pub properties: Option<Properties>,

    #[yaserde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,

    pub dependencies: Option<Dependencies>,
    pub repositories: Option<Repositories>,

    #[yaserde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,

    pub build: Option<Build>,
    pub reporting: Option<Reporting>,
    pub reports: Option<Reports>,
    pub profiles: Option<Profiles>,

    #[yaserde(attribute, prefix = "xsi", rename = "schemaLocation")]
    pub schema_location: Option<String>,

    #[yaserde(attribute, rename = "child.project.url.inherit.append.path")]
    pub child_project_url_inherit_append_path: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Parent {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,

    #[yaserde(rename = "relativePath")]
    pub relative_path: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Organization {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Licenses {
    #[yaserde(rename = "license")]
    pub licenses: Vec<License>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
    pub distribution: Option<String>,
    pub comments: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Developers {
    #[yaserde(rename = "developer")]
    pub developers: Vec<Developer>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Developer {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub organization: Option<String>,

    #[yaserde(rename = "organizationUrl")]
    pub organization_url: Option<String>,

    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    pub properties: Option<Properties>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Roles {
    #[yaserde(rename = "role")]
    pub roles: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Contributors {
    #[yaserde(rename = "contributor")]
    pub contributors: Vec<Contributor>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Contributor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub organization: Option<String>,

    #[yaserde(rename = "organizationUrl")]
    pub organization_url: Option<String>,

    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    pub properties: Option<Properties>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct MailingLists {
    #[yaserde(rename = "mailing_list")]
    pub mailing_lists: Vec<MailingList>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct MailingList {
    pub name: Option<String>,
    pub subscribe: Option<String>,
    pub unsubscribe: Option<String>,
    pub post: Option<String>,
    pub archive: Option<String>,

    #[yaserde(rename = "otherArchives")]
    pub other_archives: Option<OtherArchives>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct OtherArchives {
    #[yaserde(rename = "otherArchive")]
    pub other_archives: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Prerequisites {
    pub maven: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Modules {
    #[yaserde(rename = "module")]
    pub modules: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Scm {
    pub connection: Option<String>,

    #[yaserde(rename = "developerConnection")]
    pub developer_connection: Option<String>,

    pub tag: Option<String>,
    pub url: Option<String>,

    #[yaserde(rename = "child.scm.connection.inherit.append.path")]
    pub child_scm_connection_inherit_append_path: Option<String>,

    #[yaserde(rename = "child.scm.developerConnection.inherit.append.path")]
    pub child_scm_developer_connection_inherit_append_path: Option<String>,

    #[yaserde(rename = "child.scm.url.inherit.append.path")]
    pub child_scm_url_inherit_append_path: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct IssueManagement {
    pub system: Option<String>,
    pub url: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct CiManagement {
    pub system: Option<String>,
    pub url: Option<String>,
    pub notifiers: Option<Notifiers>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Notifiers {
    #[yaserde(rename = "notifier")]
    pub notifiers: Vec<Notifier>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Notifier {
    #[yaserde(rename = "type")]
    pub notifier_type: Option<String>,

    pub address: Option<String>,

    #[yaserde(rename = "sendOnError")]
    pub send_on_error: Option<bool>,

    #[yaserde(rename = "sendOnFailure")]
    pub send_on_failure: Option<bool>,

    #[yaserde(rename = "sendOnSuccess")]
    pub send_on_success: Option<bool>,

    #[yaserde(rename = "sendOnWarning")]
    pub send_on_warning: Option<bool>,

    pub configuration: Option<Configuration>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct DistributionManagement {
    pub repository: Option<DeploymentRepository>,

    #[yaserde(rename = "snapshotRepository")]
    pub snapshot_repository: Option<DeploymentRepository>,

    pub site: Option<Site>,
    pub relocation: Option<Relocation>,

    #[yaserde(rename = "downloadUrl")]
    pub download_url: Option<String>,

    pub status: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct DeploymentRepository {
    #[yaserde(rename = "uniqueVersion")]
    pub unique_version: Option<bool>,

    pub releases: Option<RepositoryPolicy>,
    pub snapshots: Option<RepositoryPolicy>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub layout: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct RepositoryPolicy {
    pub enabled: Option<String>,

    #[yaserde(rename = "updatePolicy")]
    pub update_policy: Option<String>,

    #[yaserde(rename = "checksumPolicy")]
    pub checksum_policy: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Site {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,

    #[yaserde(rename = "child.site.url.inherit.append.path")]
    pub child_site_url_inherit_append_path: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Relocation {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
    pub message: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct DependencyManagement {
    pub dependencies: Option<Dependencies>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Dependencies {
    #[yaserde(rename = "dependency")]
    pub dependencies: Vec<Dependency>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Dependency {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
    pub classifier: Option<String>,

    #[yaserde(rename = "type")]
    pub dependency_type: Option<String>,

    pub scope: Option<String>,

    #[yaserde(rename = "systemPath")]
    pub system_path: Option<String>,

    pub optional: Option<String>,
    pub exclusions: Option<Exclusions>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Exclusions {
    #[yaserde(rename = "exclusion")]
    pub exclusions: Vec<Exclusion>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Exclusion {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Repositories {
    #[yaserde(rename = "repository")]
    pub repositories: Vec<Repository>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct PluginRepositories {
    #[yaserde(rename = "pluginRepository")]
    pub plugin_repositories: Vec<Repository>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Repository {
    pub releases: Option<RepositoryPolicy>,
    pub snapshots: Option<RepositoryPolicy>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub layout: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Build {
    #[yaserde(rename = "defaultGoal")]
    pub default_goal: Option<String>,

    pub directory: Option<String>,

    #[yaserde(rename = "finalName")]
    pub final_name: Option<String>,

    pub filters: Option<Filters>,
    pub resources: Option<Resources>,

    #[yaserde(rename = "testResources")]
    pub test_resources: Option<TestResources>,

    #[yaserde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,

    pub plugins: Option<Plugins>,

    #[yaserde(rename = "sourceDirectory")]
    pub source_directory: Option<String>,

    #[yaserde(rename = "scriptSourceDirectory")]
    pub script_source_directory: Option<String>,

    #[yaserde(rename = "testSourceDirectory")]
    pub test_source_directory: Option<String>,

    #[yaserde(rename = "outputDirectory")]
    pub output_directory: Option<String>,

    #[yaserde(rename = "testOutputDirectory")]
    pub test_output_directory: Option<String>,

    pub extensions: Option<Extensions>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Filters {
    #[yaserde(rename = "filter")]
    pub filters: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Resources {
    #[yaserde(rename = "resource")]
    pub resources: Vec<Resource>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct TestResources {
    #[yaserde(rename = "testResource")]
    pub test_resources: Vec<Resource>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Resource {
    #[yaserde(rename = "targetPath")]
    pub target_path: Option<String>,

    pub filtering: Option<String>,
    pub directory: Option<String>,

    #[yaserde(rename = "includes")]
    pub includes: Vec<Includes>,

    #[yaserde(rename = "excludes")]
    pub excludes: Vec<Excludes>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Includes {
    #[yaserde(rename = "include")]
    pub includes: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Excludes {
    #[yaserde(rename = "exclude")]
    pub excludes: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct PluginManagement {
    pub plugins: Option<Plugins>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Plugins {
    #[yaserde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Plugin {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
    pub extensions: Option<String>,
    // pub goals: Option<OldGoals>,
    pub inherited: Option<String>,
    pub configuration: Option<Configuration>,
    pub dependencies: Option<Dependencies>,
    pub executions: Option<PluginExecutions>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct PluginExecutions {
    #[yaserde(rename = "execution")]
    pub executions: Vec<PluginExecution>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct PluginExecution {
    pub id: Option<String>,
    pub goals: Option<Goals>,
    pub phase: Option<String>,
    pub inherited: Option<String>,
    pub configuration: Option<Configuration>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Goals {
    #[yaserde(rename = "goal")]
    pub goals: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Extensions {
    #[yaserde(rename = "extension")]
    pub extensions: Vec<Extension>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Extension {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Reporting {
    #[yaserde(rename = "outputDirectory")]
    pub output_directory: Option<String>,

    #[yaserde(rename = "excludeDefaults")]
    pub exclude_defaults: Option<String>,

    pub plugins: Option<ReportPlugins>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ReportPlugins {
    #[yaserde(rename = "reportPlugin")]
    pub plugins: Vec<ReportPlugin>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ReportPlugin {
    #[yaserde(rename = "groupId")]
    pub group_id: Option<String>,

    #[yaserde(rename = "artifactId")]
    pub artifact_id: Option<String>,

    pub version: Option<String>,
    pub inherited: Option<String>,
    pub configuration: Option<Configuration>,

    #[yaserde(rename = "reportSets")]
    pub report_sets: Option<ReportSets>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ReportSets {
    #[yaserde(rename = "reportSet")]
    pub report_sets: Vec<ReportSet>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ReportSet {
    pub id: Option<String>,
    pub reports: Option<Reports>,
    pub inherited: Option<String>,
    pub configuration: Option<Configuration>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Reports {
    #[yaserde(rename = "report")]
    pub reports: Vec<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Profiles {
    #[yaserde(rename = "profile")]
    pub profiles: Vec<Profile>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Profile {
    pub id: Option<String>,
    pub activation: Option<Activation>,
    pub modules: Option<Modules>,

    #[yaserde(rename = "distributionManagement")]
    pub distribution_management: Option<DistributionManagement>,

    pub properties: Option<Properties>,

    #[yaserde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,

    pub dependencies: Option<Dependencies>,
    pub repositories: Option<Repositories>,

    #[yaserde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<Repositories>,

    pub build: Option<BuildBase>,
    pub reporting: Option<Reporting>,
    // pub reports: Option<OldReports>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct Activation {
    #[yaserde(rename = "activeByDefault")]
    pub active_by_default: Option<bool>,

    pub jdk: Option<String>,
    pub os: Option<ActivationOS>,
    pub property: Option<ActivationProperty>,
    pub file: Option<ActivationFile>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ActivationOS {
    pub name: Option<String>,
    pub family: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ActivationProperty {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct ActivationFile {
    exists: Option<String>,
    missing: Option<String>,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(namespace = "http://maven.apache.org/POM/4.0.0")]
pub struct BuildBase {
    #[yaserde(rename = "defaultGoal")]
    pub default_goal: Option<String>,

    pub resources: Option<Resources>,

    #[yaserde(rename = "testResources")]
    pub test_resources: Option<TestResources>,

    pub directory: Option<String>,

    #[yaserde(rename = "finalName")]
    pub final_name: Option<String>,

    pub filters: Option<Filters>,

    #[yaserde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,

    pub plugins: Option<Plugins>,
}

macro_rules! def_map_type {
    ($name:ident, $element:literal) => {
        pub struct $name(Map<String, Value>);

        impl YaDeserialize for $name {
            fn deserialize<R: Read>(reader: &mut YaDeserializer<R>) -> Result<Self, String> {
                use xml::reader::XmlEvent;

                let start_name = match reader.peek()?.to_owned() {
                    XmlEvent::StartElement { name, .. } => {
                        if let Some(ns) = &name.namespace {
                            match ns.as_str() {
                                "http://maven.apache.org/POM/4.0.0" => {}
                                bad_ns => {
                                    return Err(format!(
                                        "bad namespace for {} found {}",
                                        $element, bad_ns
                                    ))
                                }
                            }
                        }

                        reader.next_event()?;
                        name
                    }
                    event => return Err(format!("unexpected {event:?}")),
                };

                let map = deserialize_map(reader)?;

                match reader.peek()? {
                    XmlEvent::EndElement { name } => {
                        if name != &start_name {
                            return Err(format!("unexpected end of element: {name}"));
                        }
                    }
                    event => return Err(format!("unexpected {event:?}")),
                }

                Ok(Self(map))
            }
        }

        impl YaSerialize for $name {
            fn serialize<W: Write>(&self, writer: &mut YaSerializer<W>) -> Result<(), String> {
                use xml::writer::XmlEvent;

                writer
                    .write(
                        XmlEvent::start_element($element)
                            .default_ns("http://maven.apache.org/POM/4.0.0")
                            .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance"),
                    )
                    .map_err(|e| e.to_string())?;

                serialize_map(&self.0, writer)?;

                writer
                    .write(XmlEvent::end_element())
                    .map_err(|e| e.to_string())?;

                Ok(())
            }

            fn serialize_attributes(
                &self,
                attributes: Vec<OwnedAttribute>,
                namespace: Namespace,
            ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
                Ok((attributes, namespace))
            }
        }
    };
}

def_map_type!(Properties, "properties");
def_map_type!(Configuration, "configuration");
// def_map_type!(OldGoals, "goals");
// def_map_type!(OldReports, "reports");

pub enum Value {
    String(String),
    Map(Map<String, Value>),
}

impl YaDeserialize for Value {
    fn deserialize<R: Read>(reader: &mut YaDeserializer<R>) -> Result<Self, String> {
        use xml::reader::XmlEvent;

        let value = match reader.peek()?.to_owned() {
            XmlEvent::Characters(s) => {
                reader.next_event()?;
                Self::String(s)
            }
            XmlEvent::StartElement { .. } => {
                let map = deserialize_map(reader)?;
                Self::Map(map)
            }
            event => return Err(format!("unexpected {event:?}")),
        };

        Ok(value)
    }
}

impl YaSerialize for Value {
    fn serialize<W: Write>(&self, writer: &mut YaSerializer<W>) -> Result<(), String> {
        use xml::writer::XmlEvent;

        match self {
            Self::String(s) => writer
                .write(XmlEvent::characters(s))
                .map_err(|e| e.to_string()),
            Self::Map(map) => serialize_map(map, writer),
        }
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

fn deserialize_map<R: Read>(reader: &mut YaDeserializer<R>) -> Result<Map<String, Value>, String> {
    use xml::reader::XmlEvent;

    let mut map = Map::new();

    loop {
        let start_name = match reader.peek()?.to_owned() {
            XmlEvent::StartElement { name, .. } => {
                reader.next_event()?;
                name
            }
            XmlEvent::EndElement { .. } | XmlEvent::EndDocument => break,
            event => return Err(format!("unexpected {event:?}")),
        };

        let value = match reader.peek()?.to_owned() {
            XmlEvent::Characters(s) => {
                reader.next_event()?;
                Value::String(s)
            }
            XmlEvent::StartElement { .. } => {
                let map = deserialize_map(reader)?;
                Value::Map(map)
            }
            XmlEvent::EndElement { .. } => Value::String(String::new()),
            event => return Err(format!("unexpected {event:?}")),
        };

        reader.expect_end_element(&start_name)?;

        map.insert(start_name.local_name, value);
    }

    Ok(map)
}

fn serialize_map<W: Write>(
    map: &Map<String, Value>,
    writer: &mut YaSerializer<W>,
) -> Result<(), String> {
    use xml::writer::XmlEvent;

    for (key, value) in map {
        writer
            .write(XmlEvent::start_element(key.as_str()))
            .map_err(|e| e.to_string())?;

        match value {
            Value::String(s) => writer
                .write(XmlEvent::characters(s))
                .map_err(|e| e.to_string())?,
            Value::Map(map) => serialize_map(map, writer)?,
        }

        writer
            .write(XmlEvent::end_element())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaserde::de::from_str;
    use yaserde::ser::{to_string_with_config, Config};

    const CONFIG: Config = Config {
        perform_indent: false,
        write_document_declaration: false,
        indent_string: None,
    };

    #[test]
    fn test_deserialize_value_string() {
        let v: Value = from_str("<answer>42</answer>").unwrap();
        match v {
            Value::Map(m) => match m.get("answer") {
                Some(v) => match v {
                    Value::String(s) => assert_eq!(s, "42"),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_deserialize_value_map() {
        let m: Value = from_str("<map><answer>42</answer></map>").unwrap();
        match m {
            Value::Map(m) => match m.get("map") {
                Some(v) => match v {
                    Value::Map(m) => match m.get("answer") {
                        Some(v) => match v {
                            Value::String(s) => assert_eq!(s, "42"),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_serialize_value() {
        let xml = to_string_with_config(
            &Value::Map(Map::from([(
                "answer".to_string(),
                Value::String("42".to_string()),
            )])),
            &CONFIG,
        )
        .unwrap();
        assert_eq!(xml, "<answer>42</answer>")
    }

    #[test]
    fn test_deserialize_with_derive() {
        #[derive(YaDeserialize)]
        struct Model {
            properties: Option<Properties>,
        }

        let m: Model = from_str("<Model><properties></properties></Model>").unwrap();
        assert!(matches!(
            m,
            Model {
                properties: Some(p)
            } if p.0.is_empty()
        ));
    }
}
