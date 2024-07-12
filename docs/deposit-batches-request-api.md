 Deposit Batches Request API
==========

ISPolitical supports requests of deposit batch information via a rest API using [basic access authentication](https://ispolitical.com/how-do-i-set-up-authentication-with-isps-api/). 

Your username will be a pipe delimited concatenation of the client account name & your integration login name. For example, the Civil War Online Donation System would use “Washington|CWODS” to submit data to the Washington account. Your same login can be used for any client accounts that have granted you access. Your password will be provided by ISPolitical staff.

If you have not integrated with ISPolitical before, you will need to contact [support@ispolitical.com](mailto:support@ispolitical.com) to get an integration account setup.

Please be aware this help file is for deposit batch requests via API. If you’d like to [add a deposit batch via the API](https://ispolitical.com/How-Do-I-Add-Deposit-Batches-via-Isp-s-API), this is a separate process.

A sample deposit batch request submission follows. If you have questions, please contact ISPolitical staff.

Submit to the API at https://app.ISPolitical.com/api/Lists/

Submit the following:

```
{Item: "Deposit Batch"}
```

The following information will be returned:

1) Date

2) Amount

3) Name

4) Number

5) Dropbox Pathway

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« Budget Category Request API](/Budget-Category-Request-API)  
[About System Options in ISP »](/About-System-Options-in-ISP)