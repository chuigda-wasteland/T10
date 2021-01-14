[![Design doc](https://img.shields.io/badge/design%20doc-doc47-brightgreen)](https://github.com/Pr47/doc47)
![Not published](https://img.shields.io/badge/not-published-yellow)
![Early POC](https://img.shields.io/badge/status-early%20poc-orange)
![Don't use](https://img.shields.io/badge/dont-use-critical)

# Project-47 2021 plan

Since the previous Pr47 has failed due to certain reason, the plan for 47 project has changed a little. Here is our new plan.

## Stage-1: build the `T10` experimental project
At this stage we will re-examine the memory model of `pr47`, write several new data strcutures for function and data type bindings, and try them out with manual code instead of procedural macros.

## Stage-2: build the `su47` project
At this stage we'll write the parser/ast/vm of this project, fully implement the binding part and write the binding procedural macro.

## Stage-3: complete the ecosystem of `su47`

## Stage-4: wait for something like `unsafe_type_id`
Once rust introduces such features, procedural macro will not be necessary. Binding functions is just as easy as what user does in `rhai` or `gluon`.
