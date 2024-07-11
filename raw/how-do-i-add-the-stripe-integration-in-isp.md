 How Do I Add the Stripe Integration in ISP?
==========

***We no longer support the Stripe integration. Please see the list of other [integration and credit card processing options](https://ispolitical.com/What-Are-the-Credit-Card-Processing-Options-at-ISP) that work very well with ISP.***

Many of our integration partners offer robust and highly effective systems to integrate with your ISP database. While we highly suggest using those, we also support using your own Stripe account, a more limited infrastructure platform, for integration purposes.

The Stripe Integration page in ISP allows you to create Stripe donation pages within ISP that are linked to your Stripe account.

Please be aware that a Stripe account is required for the use of this feature.

In ISP, the Stripe Integration option under the Integrations tab will first place you on the Donation Pages screen where you’ll see all of the donation pages you’ve created.

If you have not created any pages yet, you can click the Add Donation Page button below. However, there are still steps you’ll be required to take first before you can a donation page.

At the bottom of the page you’ll see other navigation buttons, Manage Stripe Connections and Manage Donation Emails.

First start by clicking the Manage Stripe Connections button. On the resulting page, click the Add Stripe Detail button located at the bottom of the page. A window will appear only with the Filers in your own account, as well as fields for Public and Private keys, including test keys. These keys are found in your Stripe account. 

Take the following steps in your Stripe account to gather the key information you need.

1. Go to the Menu, then click on Developers, then on API Keys
2. This will show your publishable key. Click Reveal live key token to get it
3. For test keys, click on View test data at the top right
4. Repeat Step 2 above to retrieve the test keys

You’ll also see Mode options for Live and Test. If you would like to first use the page as a testing ground to ensure it works, select Test mode. Once you have worked out any kinks, you can change the mode to Live and make it available to the public. Test credit card numbers can be found here.

Once you’ve added all the necessary details, click Add Stripe Detail.

At this point, return to the Donations Page and create your donation page.

When new donations are processed, we pull that data from Stripe via an API. If it is a recurring monthly donation, Stripe will provide us with the new donation each month. From our standpoint, nothing in the information we receive indicates it being recurring. We also don’t support any sort of pre-entry because recurring donations sometimes fail or are canceled.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About Integrations in ISP](/About-Integrations-in-ISP)  
[« How Do I Set Up The Raise The Money Integration?](/How-do-I-Set-Up-The-Raise-The-Money-Integration)  
[How Do I Add the Targeted Victory Integration? »](/How-to-Create-the-Targeted-Victory-Integration)