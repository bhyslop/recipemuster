# Recipe Bottle Administration - Claude Instructions

## Activity Menu

### 1. MIND-Guided RBGS Updates
When working on `rbw-RBGS-GoogleSpec.adoc`, follow MIND-ROE principles from `a-roe-MIND-cmodel-semantic.adoc`:

- **Linked Terms**: Use proper cross-references like `{at_user}` on separate lines
- **RB-Specific**: Focus on Recipe Bottle instances, not generic Google concepts
- **Role vs Service Account**: Emphasize user roles with service accounts as implementation
- **RBRA Files**: Remember Mason has no RBRA (cloud-only), others do
- **Precise Definitions**: Each term anchors meaning within RB domain

Key files:
- Primary: `/workspace/recipebottle-admin/rbw-RBGS-GoogleSpec.adoc`
- Reference: `/workspace/cnmp_CellNodeMessagePrototype/lenses/a-roe-MIND-cmodel-semantic.adoc`

### 3. Tool Development
Working on shell scripts in `Tools/` directory for RB operations.

## Default Behavior
Unless specified otherwise, assume **Activity 1 (MIND-Guided)** when working on `.adoc` files.

## Commands
- `npm run lint` - Not applicable (AsciiDoc project)
- `npm run typecheck` - Not applicable (AsciiDoc project)