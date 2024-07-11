 How Do I Set up the Anedot Integration?
==========

Our integration partners sometimes make updates to their system that we are unaware of. If the steps provided in our help file don’t match what you see on their site, please contact our support team and we will assist you in getting the integration properly set up.

Anedot is a donation platform that provides tools to raise more money including one click Instant Donations, customizable recurring options, post-donation upsells, donor recovery, and more. When integrated with ISP, you’ll be able to receive donor data and information quickly.

How do I set up the Anedot Integration?
----------

Setting up the Anedot integration in ISP happens in two stages, first within ISP, then the process is completed in your Anedot account.  

When new donations are processed, we pull that data from Anedot via an API. If it is a recurring monthly donation, Anedot will provide us with the new donation each month. From our standpoint, nothing in the information we receive indicates it being recurring. We also don’t allow for any sort of pre-entry because recurring donations sometimes fail or are canceled.

In ISP:

1. Under the Integrations tab, click Add/Remove Integrations.
2. On the resulting Integrations screen, you will see a list of existing integrations in two sections: Installed and Available.
3. Locate and click the Anedot integration in the Available section. It will then shift to the Installed section.

Next, in Anedot:  

Log into your Anedot account and complete the process there. Anedot’s [help file](https://help.anedot.com/en/articles/9044458-ispolitical-isp) will walk you through the process on the Anedot site.  

You will need your ISP Account Name to complete the integration on Anedot. Make sure to use your short account name (the name at the top left of your screen next to the text ISP).  

Once you’ve completed the process in both ISP and Anedot, the integration will be set up. The system will now automatically create an Anedot organization record in the account with Anedot’s details and your donation data from Anedot will be automatically sent to your [Pending Imports](https://ispolitical.com/what-are-pending-imports-and-how-do-i-process-them/) on your Dashboard.

You can control how your transactions come into ISP through the Auto-Import settings on the [ai@ISP page](https://ispolitical.com/what-is-ai-isp/).   

How can I use Budget Category customization?
----------

The Customize Budget Category section on the Anedot Settings page allows you to determine which Budget Category is applied to a donation no matter the form used. You can use a Budget Category already in your database or select the Create children of default category Unmapped option to create new Budget Categories that match the source codes ISP receives from Anedot. This allows for using a generic main Budget Category, e.g., Anedot Donations, while capturing some nuance of different Anedot Pages.

You can also set up Anedot forms in ISP to map to Budget Categories. In the Per Form Categories section on the Anedot Settings page, add the Form Name and the Budget Category to be associated with the transactions that come in via this specific form. Please note that the Form Name must be a written name, as opposed to the URL for the form. This Per Form Categories setup will ensure that Anedot donations map automatically to the chosen Budget Categories via the integration import.

How do I link a donation page?
----------

Navigate to the Integrations tab and click the Anedot option. This page will inform you if the Anedot integration has been enabled, as well as if transactions have been received. If so, you’ll also see a count of how many have been received since the first Anedot transfer.  

Below the integration status and transaction count is Account UID. This allows you to link your Anedot donation page. The easiest way to locate your UID is to take it from the URL address while in your Anedot account and on the Dashboard. It will be a string of letters and numbers and will always begin with the letter a.  

Once the UID has been added and your donation page linked, you’ll be able to add donations directly through ISP. This option will be found in two locations: Individual Search Results and Add Transaction.  

In Individual Search Results, after a search is performed and the results appear, click the hot dog menu and select Enter Credit Card: Anedot Individual Record Details: When in the record, click the Add button at the bottom of the screen and select Enter Credit Card: Anedot.  

When the link opens, the entity’s basic contact info will auto-populate and you’ll see fields to enter in donation information. The donation will be processed by Anedot and come into ISP via the regular integration method.  

Please note that linking another PAC as a conduit is currently not supported by ISP. In this instance, contributions will either need to be manually entered or imported via the Quick Imports tool.

Campaign vs Action Pages
----------

Anedot has two different types of integration pages: Action Pages and Campaign.  

As for functionality, Action Pages will contain more. However, we have no preference as to which one is used, as both integrate with ISP. You can use either or both. To be safe, we do suggest turning both integration options on.  

Please note only Campaign Pages have a Push Unsent Donations option, which will push any historical data over to ISP.

Troubleshooting Problems
----------

Below are some common problems you may encounter.

1. My donors are coming through but not the Occupation/Employer information: In Anedot, there are ways to set up Custom Fields for the form. However, if there are issues with the set up, it can cause information to not be sent over to ISP. Fortunately, Anedot also has built in Occupation/Employer fields for forms. If you used Custom Fields for Occupation/Employer and this info is not coming over to ISP, remove the Custom Fields from the form and use the Anedot-built fields instead.
2. I’m getting duplicate transactions from Anedot: It’s possible there are two ISP connections in Anedot. This can cause items to come over twice. Access your Anedot account and review the connections, then delete the one not needed.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About Integrations in ISP](/About-Integrations-in-ISP)  
[« How Do I Use the ActBlue Quick Import?](/How-Do-I-Use-the-ActBlue-Quick-Import)  
[How Do I Set up the Constant Contact Integration? »](/How-Do-I-Set-up-the-Constant-Contact-Integration)