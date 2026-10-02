// src/dispatch/techniques_spec.rs
// 110项玄学占卜技法专属元数据、入参规约与调度前置注册表

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct TechniqueMeta {
    pub id: usize,
    pub tool: &'static str,
    pub name_zh: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub params: &'static [&'static str],
    pub example_input_json: &'static str,
    pub expect_desc: &'static str,
}

impl TechniqueMeta {
    pub fn get_example_value(&self) -> Value {
        serde_json::from_str(self.example_input_json).unwrap_or(Value::Null)
    }
}

pub const ALL_110_TECHNIQUES: &[TechniqueMeta] = &include!("techniques_meta_data.inc");

/// 根据技法标识符获取元数据规约
pub fn get_technique_meta(tool: &str) -> Option<&'static TechniqueMeta> {
    ALL_110_TECHNIQUES.iter().find(|t| t.tool == tool)
}

/// 获取所有110项技法分类列表
pub fn get_all_categories() -> Vec<&'static str> {
    let mut cats: Vec<&'static str> = Vec::new();
    for t in ALL_110_TECHNIQUES {
        if !cats.contains(&t.category) {
            cats.push(t.category);
        }
    }
    cats
}

/// 根据分类筛选技法
pub fn get_techniques_by_category(category: &str) -> Vec<&'static TechniqueMeta> {
    ALL_110_TECHNIQUES.iter().filter(|t| t.category == category).collect()
}

/// 生成给其他 AI 开发或调用的统一描述文档 / 注册表快照
pub fn generate_ai_registry_json() -> Value {
    let list: Vec<Value> = ALL_110_TECHNIQUES
        .iter()
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "tool": t.tool,
                "name_zh": t.name_zh,
                "category": t.category,
                "description": t.description,
                "params": t.params,
                "example_input": t.get_example_value(),
                "expect_desc": t.expect_desc,
            })
        })
        .collect();

    serde_json::json!({
        "total_techniques": ALL_110_TECHNIQUES.len(),
        "categories": get_all_categories(),
        "techniques": list,
        "note": "110项玄学占卜统一调度前置模块，包含完整入参规范与测试用例示例"
    })
}
