 How Do I Add Volunteers With Flags Via ISP’s API?
==========

ISPolitical supports submissions of online donations, fees and other expenses, and other public forms via a rest API. If this is your first time with API submissions, please read up on the [authentication requirements and process](https://ispolitical.com/How-Do-I-Set-Up-Authentication-with-ISP-s-API/) in ISP.

Use our API sample submission below if you have volunteers you’d like to submit via our API. Please also be sure the Flag you include in your submission already exists in ISP. If you have need for additional fields or questions, please contact ISPolitical staff.

**Volunteer Sample**
----------

```
{
   "FirstName":"Betsy",
   "LastName":"Ross",
   "Emails":[
      {
         "EmailAddress":"betsy@creativeupholstery.com"
      }
   ],
   "Flags":[
      {
         "FlagName":"Sewing"
      },
      {
         "FlagName":"Graphic Design"
      }
   ]
}
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« How Do I Add Deposit Batches via ISP’s API?](/How-Do-I-Add-Deposit-Batches-via-ISP-s-API)  
[API Sample Code: Javascript »](/API-Sample-Code-Javascript)