install:
   cargo make --profile release install
package:
   cargo make --profile release package
package-all:
   cargo make --profile release package-all
   