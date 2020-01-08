module Test.Generated.Main766805528 exposing (main)

import Tests

import Test.Reporter.Reporter exposing (Report(..))
import Console.Text exposing (UseColor(..))
import Test.Runner.Node
import Test

main : Test.Runner.Node.TestProgram
main =
    [     Test.describe "Tests" [Tests.tests] ]
        |> Test.concat
        |> Test.Runner.Node.run { runs = Nothing, report = (ConsoleReport UseColor), seed = 37349304826744, processes = 4, paths = ["/Users/estysdesu/OneDrive - University of Cincinnati/projects/exercism/elm/two-fer/tests/Tests.elm"]}