use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::JsForVariableDeclaration;
use rome_js_syntax::JsForVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsForVariableDeclaration;

impl FormatNodeRule<JsForVariableDeclaration> for FormatJsForVariableDeclaration {
    fn fmt_fields(&self, node: &JsForVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForVariableDeclarationFields {
            async_token,
            kind_token,
            declarator,
        } = node.as_fields();

        if let Some(async_token) = async_token {
            write!(f, [async_token.format(), space()])?;
        }

        write![
            f,
            [group(&format_args![
                kind_token.format(),
                space(),
                declarator.format()
            ])]
        ]
    }
}
