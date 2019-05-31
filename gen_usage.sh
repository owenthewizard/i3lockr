#!/bin/sh

set -euo pipefail

cat > USAGE.md <<EOF 
# Usage

See the help:

\`\`\`
EOF

cargo run -- --help >> USAGE.md

cat >> USAGE.md <<EOF
\`\`\`

Items marked \`[NYI]\` are \`Not Yet Implemented\` and may function partially or not at all!
EOF
