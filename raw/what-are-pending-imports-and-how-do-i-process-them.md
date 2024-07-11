 What Are Pending Imports and How Do I Process Them?
==========

Nearly all of the Integrations in ISP use the Pending Imports system. This means when transactions are created in other programs integrated with your ISP database, the transactions will appear in an import queue found on your Dashboard.

If there are any transactions and records to import, there will be a small section on your Dashboard indicating this. In this section is a toggle switch for Email Notifications. This turns sets the ability to receive notifications of pending transactions to be imported. If you do not see this section on your Dashboard, this indicates you have no transactions or data to import.

When you click the Click to Process button, you’ll be directed to a new screen called Pending Import Matches. Here you’ll see pending imports and potential matches with already existing records in your database.

### Import Options ###

At the top of the screen are four drop down menus: Set Non Pre-Set Financial Accounts, Election, Budget Category, and Event. These options are determined by what’s already in your database on those pages. Please note that Election and Budget Category are required. Below those menus are options to mark the transactions as thanked upon import and import the transactions as Undeposited.

When it comes to these menus and options, they are considered sticky. This means the options you use now will automatically be there for the next import. You can change them each time but what you change to now will be considered the preferred options next time.

### Pending Imports ###

In the Pending Import section will be the entity and transaction information, as well as any potential record matches. If there are potential matches, you will have options to Import, Add As New, Skip For Now, and Delete & Do Not Import. There is also an option for Custom Import which allows you to have more control over what information is used when combining the matched records upon import.

If you want to select one single option for all imports, you’ll see an Options dropdown in the top right corner of this section. This will set the same import option for all pending import transactions and individuals.

When you have selected your options and are ready for import, click the Process Import button at the bottom of the screen.

Keep in mind when it comes to imports:

Temporary duplicate contact information is normal & expected with imports. They should get automatically cleaned up as part of the cleanup process. This typically runs within an hour of the import.

About AI Imports
----------

1.  ISP will automatically filter out known common fake email addresses during the import process. For example: noreply@noreply.com and test@test.com. If you have common addresses that should be blocked from import to your database, please let us know.
2. AI’s import assumption is that newly provided information is correct. For example: If John Doe previously worked at “American Airlines” and then went to work at “Delta Airlines”, AI would assume that the new information provided was correct.
3. If a budget category is submitted via the API, then that takes precedence over a selected budget category when doing an import. That applies to both AI imports and human imports.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ How Do I Get Started in ISP?](/How-Do-I-Get-Started-in-ISP)  
[« What Are the Key Statistics on the Dashboard?](/What-Are-the-Key-Statistics-on-the-Dashboard)  
[What Do the Different Button Colors Mean? »](/What-Do-the-Different-Button-Colors-Mean)