 How Do I Add Political Committees via ISP’s API?
==========

ISPolitical supports submissions of online donations, fees and other expenses, and other public forms via a rest API. If this is your first time with API submissions, please read up on the [authentication requirements and process](https://ispolitical.com/How-Do-I-Set-Up-Authentication-with-ISP-s-API/) in ISP.

Below is an example of the needed JSON for adding a Political Committee and associated donation via the API

**Political Committee Sample**
----------

```
{
   "FullName":"People for Sunshine and Rainbows",
   "EntityType":"Political Committee",
   "AddressType":"Work",
   "Line1":"325 Sunshine Blvd.",
   "Line2":"",
   "City":"Sunnyvale",
   "State":"CA",
   "ZipCode":"94086",
   "Notes":"Thanks for doing positive things!",
   "Source":"Fall Fundraiser",
   "PoliticalCommittee":
   {
      "CommitteeType":"Political Action Committee (PAC)",
      "IdNumber":"C00123456",
      "Jurisdiction":"Federal",
      "DateQualified":"2022-11-01",
      "EIN":"12-3456789",
      "Controlled":false,
      "Sponsored":false
   },
   "Transactions":[
      {
         "Amount":"500",
         "Date":"2023-01-02",
         "NoteForInternal":"Friends of Abe Club"
      }
   ],
   "Emails":[
      {
         "EmailAddress":"happy@sunshinepac.org"
      }
   ],
   "Phones":[
      {
         "PhoneNumber":"(555) 456-1111",
         "PhoneType":"Work"
      }
   ]
}
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« How Do I Set Up Authentication with ISP’s API?](/How-Do-I-Set-Up-Authentication-with-ISP-s-API)  
[What Fields Are Available for API Submissions? »](/What-Fields-Are-Available-for-API-Submissions)