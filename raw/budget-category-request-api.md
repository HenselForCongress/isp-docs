 Budget Category Request API
==========

ISPolitical supports requests of budget categories via a rest API using [basic access authentication](https://ispolitical.com/how-do-i-set-up-authentication-with-isps-api/).

Your username will be a pipe delimited concatenation of the client account name & your integration login name. For example, the Civil War Online Donation System, would use Washington|CWODS to submit data to the Washington account. Your same login can be used for any client accounts that have granted you access. Your password will be provided by ISPolitical staff.

If you have not integrated with ISPolitical before, you will need to contact Support@ispolitical.com to get an integration account setup.

A sample budget category request submission follows. If you have questions, please contact ISPolitical staff.

Submit to the API at https://app.ISPolitical.com/api/Lists/

Submit the following:

```
{Item: "Budget Category"}
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-API)  
[« API Sample Code: JavaScript](/API-Sample-Code-JavaScript)  
[Deposit Batches Request API »](/Deposit-Batches-Request-API)