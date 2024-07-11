 How Do I Isolate Out-Of-State Income Using Advanced Search?
==========

If you’d like to figure out the amount of donations you’ve received that are from out of state, you can do so with Advanced Search.

1. Click on the Add/Search menu, then click on Advanced Search.  
2. Your first rule is going to set up your Entities. You can choose to limit this to Individuals and Organizations, or you can include everything.

* Area: Entities
* Field: Entity Type
* Relationship: Is Any Of
* Value: Individual, Organization, Other Entity

3. Now, we need to isolate out-of-state addresses.

* Area: Addresses
* Field: State
* Relationship: Is Not Any Of
* Value: [Your state]

4. Select Save & Run Search and give this search a name.  
5. After you save the search, navigate to Reports, then select Financial.  
6. In the Report drop-down menu, select the Transactions report.  
7. Select your chosen date range.  
8. In the Transaction Type box, input Monetary Contribution.  
9. In the Advanced Searches field near the bottom, select the Advanced Search you saved earlier. If you can’t find the Advanced Searches field, you may have to scroll down. If you still don’t see it, select Add Row near the top of this page, and add the Advanced Searches row.  
10. Generate.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ What Are Advanced Search Recipes?](/What-Are-Advanced-Search-Recipes)  
[« How Do I Exclude Results Based on Flags Using Advanced Search?](/How-Do-I-Exclude-Results-Based-on-Flags-Using-Advanced-Search)  
[How Do I Find All PACs That Gave in the Primary and General Cycle? »](/How-Do-I-Find-All-PACs-That-Gave-in-the-Primary-and-General-Cycle)