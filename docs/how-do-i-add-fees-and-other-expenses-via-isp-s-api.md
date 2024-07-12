 How Do I Add Fees and Other Expenses via Isp’s API?
==========

ISPolitical supports submissions of online donations, fees and other expenses, and other public forms via a rest API. If this is your first time with API submissions, please read up on the [authentication requirements and process](https://ispolitical.com/How-Do-I-Set-Up-Authentication-with-ISP-s-API/) in ISP.

Below is an API sample submission for an expense. Additional data may be submitted. Please see our [full list of available fields](https://ispolitical.com/what-fields-are-available-for-isps-api-submissions/).

**Fee Sample**
----------

```
{
   "Company":"Credit Card Processing Solutions LLC",
   "EntityType":"Company",
   "Line1":"5555 Green Blvd",
   "Line2":"Suite 500",
   "City":"The Lakes",
   "State":"NV",
   "ZipCode":"88901",
   "Transactions":[
      {
         "Amount":"62.23",
         "Date":"2020-01-07",
         "NoteForInternal":"Processing Fees",         "NoteForCompliance":"Credit Card Fees",         "ReportingCode":"011",
         "TransactionType":"Expense"
      }
   ]
}
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« How Do I Add Online Donations via ISP’s API?](/How-Do-I-Add-Online-Donations-via-ISP-s-API)  
[How Do I Add Deposit Batches via ISP’s API? »](/How-Do-I-Add-Deposit-Batches-via-ISP-s-API)