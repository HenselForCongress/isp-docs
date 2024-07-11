 How Do I Set up an ActBlue Integration?
==========

Our integration partners sometimes make updates to their system that we are unaware of. If the steps provided in our help file don’t match what you see on their site, please contact our support team and we will assist you in getting the integration properly set up.

Setting up your ActBlue integration is a multi-step process, including contacting ActBlue via email.

When new donations are processed, we pull that data from ActBlue via an API. If it is a recurring monthly donation, ActBlue will provide us with the new donation each month. From our standpoint, nothing in the information we receive indicates it being recurring. We also don’t support any sort of pre-entry because recurring donations sometimes fail or are canceled.

When transactions come over from ActBlue, we respect the timezone that ActBlue specifies. For example, if a transaction is sent to us on August 2nd from ActBlue at 1:00 am Eastern Time, even though for us that is August 1st at 10:00 pm Pacific, the system will treat that as an August 2nd transaction.

You can control how your ActBlue transactions come into ISP through the [ActBlue integration page](https://ispolitical.com/what-are-the-actblue-settings/). 

You can also control your ActBlue transactions through the Auto-Import settings on the [ai@ISP page](https://ispolitical.com/what-is-ai-isp/).

### How do I add the ActBlue integration? ###

1. Go to the Integrations tab and select Add/Remove Integrations.

2. On the resulting page, in the Available section, click the ActBlue integration.

3. A new window will appear and ask for your ActBlue Entity ID. This can be found by logging into your ActBlue account and viewing the URL of the Dashboard. It will be a 5 digit number and look something like this: [https://secure.actblue.com/entities/XXXXX/dashboard](https://secure.actblue.com/entities/XXXXX/dashboard)

4. After adding the ActBlue Entity ID, the next step is contacting ActBlue to complete the integration process. You’ll be sent an email from ISP with the exact information you need to send to ActBlue so they can complete the setup on their end.

After you send the email to ActBlue (support@actblue.com), ActBlue donations will appear in the [Pending Imports](https://ispolitical.com/what-are-pending-imports-and-how-do-i-process-them/) section of the Dashboard within about 24 hours.

Once the integration is set up, the system will automatically create an ActBlue organization record in the account with ActBlue’s details. We also suggest manually creating an ActBlue Technical Services organization record for any fees incurred from ActBlue.

In regards to the dates on these transactions, ActBlue sends the contribution data real-time as it is made. That is part of the process they have setup for integrations overall. While the conduit check itself can have the date received, the contributions would have the date given on them.

Please note temporary duplicate contact information is normal & expected with imports. They should get automatically cleaned up as part of the cleanup process. This typically runs within an hour of the import.

### Related Information ###

* [What Are the ActBlue Settings?](https://ispolitical.com/What-Are-the-ActBlue-Settings)
* [How Do I Use the ActBlue Quick Import?](https://ispolitical.com/How-Do-I-Use-the-ActBlue-Quick-Import)

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About Integrations in ISP](/About-Integrations-in-ISP)  
[« Credit Card Failure Codes](/Credit-Card-Failure-Codes)  
[What Are the ActBlue Settings? »](/What-Are-the-ActBlue-Settings)