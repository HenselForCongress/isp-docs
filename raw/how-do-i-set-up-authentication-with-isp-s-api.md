 How Do I Set Up Authentication with ISP’s API?
==========

**Please note this information is for software developers. If you are unsure about the following information, please contact your site developer.**

Getting Started  

----------

In order to submit to ISP’s API, you need a username and password. If you have not integrated with ISP before, please contact us at [support@ispolitical.com](mailto:support@ispolitical.com) to get your integration setup and to obtain a token (also referred to as integration name) and password.

Basic Authentication  

----------

ISP uses [basic access authentication](https://en.wikipedia.org/wiki/Basic_access_authentication), slightly modified to support access to multiple accounts.

Your username will be a pipe delimited concatenation of the client account name and your integration login name. That means the format would be [Account]|[Integration] and is unique for each account data is being passed to. For example, the Civil War Online Donation System (CWODS), would use Washington|CWODS as the username to submit data to the specific account called Washington.

Together with the password, a base 64 authentication string should be posted to ISP’s API at https://app.ISPolitical.com/api/PublicForms/

Example
----------

If the integration is called CWODS, the account you are sending data to is Washington, and the password is ABC123, the submission would look like this:

Washington|CWODS:ABC123

Using base 64 encoding, that becomes V2FzaGluZ3RvbnxDV09EUzpBQkMxMjM=

If the same integration was also sending data to a different account at ISP, the base 64 encoded authorization string would be different. For example, if the second account name is Lincoln the submission would look like this:

Lincoln|CWODS:ABC123

Using base 64 encoding, that becomes TGluY29sbnxDV09EUzpBQkMxMjM=

Both examples above are from the same example integration, but the data would go into two different accounts.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[« About ISP’s API](/About-ISP-s-API)  
[How Do I Add Political Committees via ISP’s API? »](/How-Do-I-Add-Political-Committees-via-ISP-s-API)