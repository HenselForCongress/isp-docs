 How Do I Add Deposit Batches via ISP’s API?
==========

ISPolitical supports submissions of online donations and other public forms via a rest API. If this is your first time with API submissions, please read up on the [authentication requirements and process](https://ispolitical.com/How-Do-I-Set-Up-Authentication-with-ISP-s-API/) in ISP.

ISP supports the creation of deposits based on payouts sent to clients.

Please be aware this help file is for adding a deposit batch via API. [Requestion deposit batch info via the API](https://ispolitical.com/Deposit-Batches-Request-API) is a separate process.

In order to utilize this, you need to include in the Transactions section of the JSON a “UniqueIdentifier” for each contribution and expense sent. Use the API sample submission below if you have deposit batches you’d like to submit via our API. If you have need for additional fields or questions, please contact ISPolitical staff.

Submitting Contributions in Batches
----------

Example of the added “UniqueIdentifier” line. Please see the [contribution help file](https://ispolitical.com/how-do-i-add-online-donations-via-isps-api/) for a full sample.

```
"Transactions":[
   {
      "Amount":"500",
      "Date":"2020-01-07",
      "TransactionType":"Monetary Contribution",
      "UniqueIdentifier":"redleader-7divisionattack"
   }
]
```

----------

----------

Submitting Fees  

----------

Example of the added “UniqueIdentifier” line. Please see the [expense help file](https://ispolitical.com/how-do-i-add-fees-and-other-expenses-via-isps-api/) for a full sample.

```
"Transactions":[
   {
      "Amount":"62.23",
      "Date":"2020-01-07",
      "NoteForInternal":"Processing Fees",
      "TransactionType":"Expense",
      "UniqueIdentifier":"R2D2-urmyonlyhope7"
   }
]

```

Submitting the Deposit Batch  

----------

You can then send over a Deposit transaction containing the unique identifiers included on each contribution in the payout (as well as fees). The deposit info will then be used to create a batch deposit in ISP with all the transactions listed.

When using the example of a deposit submission below, note that Company *must* be Deposit and EntityType *must* be Other.

```
{
   "Company":"Deposit",
   "EntityType":"Other",
   "Transactions":[
      {
         "Amount":"437.77",
         "Date":"2020-01-07",
         "TransactionType":"Deposit",
         "Items":[
            "R2D2-urmyonlyhope7",
            "redleader-7divisionattack"
         ]
      }
   ]
}
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« How Do I Add Fees and Other Expenses via Isp’s API?](/How-Do-I-Add-Fees-and-Other-Expenses-via-Isp-s-API)  
[How Do I Add Volunteers With Flags Via ISP’s API? »](/How-Do-I-Add-Volunteers-With-Flags-Via-ISP-s-API)