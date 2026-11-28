//! Bind this host's deployed installer and roadmap to the reviewed Copier rendering.
#![allow(unexpected_cfgs)]

use vstd::prelude::*;

verus! {

pub fn deployed_files() -> (&'static str, &'static str, &'static str) {
    (
        include_str!("../../../.chrono-harness/install-tools.sh"),
        include_str!("../../../.github/roadmap.cjs"),
        include_str!("../../../.github/workflows/issue-roadmap.yml"),
    )
}

pub fn matches_render(
    installer: &String,
    roadmap: &String,
    roadmap_workflow: &String,
) -> (matches: bool)
    ensures matches == (installer@ == "0a79a172801d92e96212034f0accaf3b4af3e4ec0f056cb3250e267bd64ecc5e"@
        && roadmap@ == "86e97ab0ed63be36d37d32e7d164da4e226c6f616685ad4ce1dd75f814095f8a"@
        && roadmap_workflow@ == "2ffcde74437d601347b884741aa464f15c378d0db42461778ec556a55cf31e75"@),
{
    *installer == "0a79a172801d92e96212034f0accaf3b4af3e4ec0f056cb3250e267bd64ecc5e".to_string()
        && *roadmap == "86e97ab0ed63be36d37d32e7d164da4e226c6f616685ad4ce1dd75f814095f8a".to_string()
        && *roadmap_workflow == "2ffcde74437d601347b884741aa464f15c378d0db42461778ec556a55cf31e75".to_string()
}

}

#[cfg(not(verus_keep_ghost))]
#[test]
fn deployed_adoption_matches_the_rendered_files() {
    use sha2::{Digest, Sha256};
    let (installer, script, workflow) = deployed_files();
    let digest = |text: &str| {
        Sha256::digest(text.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    };
    assert!(matches_render(
        &digest(installer),
        &digest(script),
        &digest(workflow)
    ));
}
