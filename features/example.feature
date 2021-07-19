Feature: Example feature

  Scenario: some nonsense with a string
    Given a string with some particular value
    When I append a known suffix to the value
    Then that string is now equal to "something interesting"

  Scenario: async things
    Given some awkward async process to get a string
    And some tables being filled with data
    When we check a database we have prefilled with a string
    Then we find we somehow had the same string
