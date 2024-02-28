use json_with_comment::{self, from_str};
use serde::Deserialize;

#[test]
fn test_deserialize_ignored() {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Setting {
        name: &'static str,
        image: Option<&'static str>,
        remote_user: Option<&'static str>,
        mounts: Option<Vec<&'static str>>,
    }
    let target = r#"
        {
            "name": "Debian",
            "image": "mcr.microsoft.com/vscode/devcontainers/base:0-bullseye",
            "remoteUser": "vscode",
            "mounts": null,
            "customizations": {}, // this field is not defined in struct
            "features": {}, /* this field is not defined in struct */
        }
    "#;
    let setting = from_str::<Setting>(target).unwrap();
    assert!(matches!(
        setting,
        Setting {
            name: "Debian",
            image: Some("mcr.microsoft.com/vscode/devcontainers/base:0-bullseye"),
            remote_user: Some("vscode"),
            mounts: None
        }
    ));
}
