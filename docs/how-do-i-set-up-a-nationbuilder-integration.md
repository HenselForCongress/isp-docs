 How Do I Set up a NationBuilder Integration?
==========

Our integration partners sometimes make updates to their system that we are unaware of. If the steps provided in our help file don’t match what you see on their site, please contact our support team and we will assist you in getting the integration properly set up.

The NationBuilder integration allows you to import donation data entered on your NationBuilder site directly into your ISP database. This integration pulls data from NationBuilder with dates in the last two weeks. If there are transactions entered into NationBuilder older than two weeks, they will need to be manually imported separately.

When new donations are processed, we pull that data from NationBuilder via an API. If it is a recurring monthly donation, NationBuilder will provide us with the new donation each month. From our standpoint, nothing in the information we receive indicates it being recurring. We also don’t support any sort of pre-entry because recurring donations sometimes fail or are canceled.

### How do I add the NationBuilder integration to my database? ###

1. Under the Integrations tab, click Add/Remove Integrations.

2. On the resulting screen, scroll until you see NationBuilder in the Available section, then click it.

3. In the window that appears requesting Slug information, enter your NationBuilder account name. You can find this by going to your NationBuilder site and looking at the website name. For example: If your NationBuilder website was johnson.nation.builder.com, you would enter Johnson as the Slug.

4. After you’ve added the Slug, click Add Integration. You will be automatically redirected to the NationBuilder site for verification and approvals on their end. You will be prompted to sign in if you’re not already signed into your NationBuilder account. After signing in, click Authorize.

Once the integration is set up, the system will automatically create a NationBuilder organization record in your ISP database with NationBuilder’s details.

When data is transferred from NationBuilder to ISP, you can find it in the [Pending Imports](https://ispolitical.com/what-are-pending-imports-and-how-do-i-process-them/) section of your Dashboard. 

Please note the following:

* NationBuilder integration supports In-Kinds. The transaction must have a tracking code in NationBuilder of IN-KIND or INKIND to be distinguished from regular contributions.
* Recurring Donations: When new donations are processed, we pull that data from NationBuilder via an API. If it is a recurring monthly donation, NationBuilder will provide us with the new donation each month. From our standpoint, nothing in the info we receive indicates it being recurring. We also don’t any sort of pre-entry because recurring donations sometimes fail or are canceled.
* There is up to a 30 minute delay for incoming donations to fully process into the ISP system.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About Integrations in ISP](/About-Integrations-in-ISP)  
[« Why Are My Broadcast Emails Taking So Long to Be Delivered?](/Why-Are-My-Broadcast-Emails-Taking-So-Long-to-Be-Delivered)  
[Ninja Forms Integration Setup »](/Ninja-Forms-Integration-Setup)