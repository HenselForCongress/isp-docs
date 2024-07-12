 How Do I Find Donors Who Gave to the Primary but Not the General?
==========

Donors are an important part of any campaign, especially within every election cycle. Finding those that donated to the Primary but not to the General is a useful step in raising more funds. Using Advanced Search, you can set specific search criteria that lets you find exactly what you’re looking for.

This report will happen in several steps using different rules and criteria. First start by locating those that donated to the Primary election.

* **Area:** Contributions
* **Field:** Election
* **Relationships:** Is Any Of
* **Value:** (select the Primary election, will start with a P)

Click the Add Rule button.

Next, back in the Add Rule section:

* **Area:** Contributions
* **Field:** Election
* **Relationships:** Is Any Of
* **Value:** (select the General election, will start with a G)

This time, click the Add as Exclusion button. This will remove any donors that have made a contribution during the General. Click Run Search to generate the report and see the results on screen. Again, this report will show all donors who contributed during the Primary but not the General.

What if the donor has reached the donation limit?
----------

Sometimes the reason why someone wouldn’t donate again during the General is that they’ve already maxed out. In that case, add the following Rule to exclude donors who have reached their limit: 

* **Area:** Contributions
* **Field:** Amount
* **Relationship:** Is Less Than
* **Value:** (add contribution limit, no $ sign needed)

Click Add Rule and your search will only include donors who haven’t maxed out.

If you’re planning on running this specific search regularly, click Save & Run Search to save the criteria for future report generation.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ What Are Advanced Search Recipes?](/What-Are-Advanced-Search-Recipes)  
[« How Do I Exclude Results Based on Flags Using Advanced Search?](/How-Do-I-Exclude-Results-Based-on-Flags-Using-Advanced-Search)  
[How Do I Find Non-donors? »](/Advanced-Search-Recipe-Find-Non-Donors)