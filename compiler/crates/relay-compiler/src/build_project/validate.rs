/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::DiagnosticsResult;
use errors::try_all;
use graphql_ir::Program;
use relay_transforms::{
    disallow_reserved_aliases, disallow_typename_on_root, validate_connections,
    validate_module_names, validate_relay_directives, validate_unused_fragment_variables,
    validate_unused_variables, ConnectionInterface,
};

pub type AdditionalValidations = Box<dyn Fn(&Program) -> DiagnosticsResult<()> + Sync + Send>;

pub fn validate(
    program: &Program,
    connection_interface: &ConnectionInterface,
    additional_validations: &Option<AdditionalValidations>,
    skip_unused_fragment_variable_validation: bool,
) -> DiagnosticsResult<()> {
    try_all(vec![
        disallow_reserved_aliases(program),
        validate_unused_variables(program),
        if skip_unused_fragment_variable_validation {
            Ok(())
        } else {
            validate_unused_fragment_variables(program)
        },
        validate_connections(program, connection_interface),
        validate_relay_directives(program),
        validate_module_names(program),
        disallow_typename_on_root(program),
        if let Some(ref validate) = additional_validations {
            validate(program)
        } else {
            Ok(())
        },
    ])?;

    Ok(())
}
