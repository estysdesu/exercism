module Test.Generated.Main2697607207 exposing (main)

import Tests

import Test.Reporter.Reporter exposing (Report(..))
import Console.Text exposing (UseColor(..))
import Test.Runner.Node
import Test

main : Test.Runner.Node.TestProgram
main =
    [     Test.describe "Tests" [Tests.tests] ]
        |> Test.concat
        |> Test.Runner.Node.run { runs = Nothing, report = (ConsoleReport UseColor), seed = 117122871095305, processes = 4, paths = ["/Users/estysdesu/OneDrive - University of Cincinnati/projects/exercism/elm/two-fer/tests/Tests.elm"]}