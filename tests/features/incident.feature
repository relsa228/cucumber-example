Feature: Working with incidents

  Scenario: Saving an incident to the local file
    Given Michael wanted to create service with limit -1
    Given he wants to save an incident with name Incident_1 and content Content_1
    When he initializes the service
    When he creates file output.txt
    When he saves the incident
    Then incident is saved
