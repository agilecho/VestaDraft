use super::{entity, invalid, payload, tagged, words};
use serde::Deserialize;
use serde_json::{Value, json};
use vesta_model::Result;

const FIELD: &str = "nativeLegacyViewCurrent";
const TAG: &str = "NativeLegacyViewCurrent1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ViewInput {
    pub(super) matrix_bits: [String; 16],
    pub(super) settings_bits: [String; 4],
}

pub(super) fn install(input: ViewInput, scene_id: &str, source: &mut Value) -> Result<()> {
    let scene = source["scenes"]
        .as_array_mut()
        .and_then(|rows| rows.iter_mut().find(|scene| scene["id"] == scene_id))
        .ok_or_else(|| invalid("current native view has no installed scene"))?;
    let camera = &mut scene["cameraRaw"];
    if camera["kind"] == "Null" {
        *camera = json!({"kind":"Map","members":{}});
    }
    let members = camera["members"]
        .as_object_mut()
        .ok_or_else(|| invalid("actual scene camera state is not a map"))?;
    if members.contains_key(FIELD) {
        return Err(invalid("current native view already installed"));
    }
    let value = json!({
        "schemaVersion":"FC-vics-legacy-view-current-1","captureRole":"actual-current-native-view",
        "scene":entity("scene",scene_id),"matrix16":words(input.matrix_bits,"F64Bits"),
        "settings4":words(input.settings_bits,"F32Bits")
    });
    members.insert(FIELD.into(), tagged(TAG, &value)?);
    Ok(())
}

pub(super) fn inspect(source: &Value) -> Result<Vec<Value>> {
    let mut views = Vec::new();
    for scene in source["scenes"].as_array().into_iter().flatten() {
        let Some(node) = scene["cameraRaw"]["members"].get(FIELD) else {
            continue;
        };
        let id = scene["id"]
            .as_str()
            .ok_or_else(|| invalid("current native view scene id"))?;
        let body = payload(node)?;
        if node["tag"] != TAG || body["scene"] != entity("scene", id) {
            return Err(invalid(
                "current native view owner differs from containing scene",
            ));
        }
        views.push(body);
    }
    Ok(views)
}
