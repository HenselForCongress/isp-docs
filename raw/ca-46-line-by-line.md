 CA 460 Line by Line
==========

Applies To: California

The CA 460 report is used by recipient committees to report both expenditures and receipts. Use the line by line guide below to ensure or troubleshoot data on the report is appearing where it should.

Please note the date of election will only pull to the report if you are running a Pre-Election report (Primary or General). The date will not pull to the report if you are running a Semi-Annual Report.

### Cover Page ###

### Section 1: Type of Recipient Committee ###

This information is pulled from the Filer Committee Type within the Committee record.

* **Officeholder, Candidate Controlled Committee:** Filer Committee Type is Candidate
* **State Candidate Election Committee:** Candidate Record is affiliated with the Filer Committee Entity Record vis Relationships. Also ensure the Office Sought is one of the following:
* Governor
* Lieutenant Governor
* Secretary of State
* State Controller
* Attorney General
* State Treasurer
* Insurance Commissioner
* Superintendent of Public Instruction
* Supreme Court Justice
* State Senator
* State Assembly Person
* Board of Equalization Member
* Public Employees Retirement System
* State Appellate Court Justice

* **Recall:** Current Election is a Recall Election
* **General Purpose Committee:** Filer Committee type is NOT Candidate or Ballot Measure. Also ensure there is no individual linked to your Filer Entity Record with either the Supported Candidate or Opposed Candidate relationship.
* **Sponsored:** Sponsored is checked in the Committee Info on the Filer Entity Record
* **Small Contributor Committee:** Filer Committee type is Small Contributor Committee
* **Political Party / Central Committee:** Filer Committee type is Party
* **Primarily Formed Ballot Measure:** Filer Committee type is Ballot Measure
* **Controlled:** Controlled box is checked in the Committee Info on the Filer Entity Record
* **Primarily Formed Candidate/Officeholder Committee:** Individual is linked to your Filer Entity Record with either the Supported Candidate or Opposed Candidate relationship.

### Section 2: Type of Statement  
 ###

This is determined by the Report Period dropdown menu on the criteria screen when generating the report.

### Section 3: Committee Information ###

* Committee contact info pulls directly from the committee record itself. Address marked as Primary will pull to the first instance of address; Address marked is Mailing will pull to the second instance (if different).
* Treasurer(s) / Assistant Treasurer(s) must be linked via Relationships on the Filer Committee entity record.

### Section 4: Verification ###

* **Executed on:** The date you generate the report
* **Signers:** Based on the Relationships found on the Filer Committee entity record. These can be selected during the report generation process. You can select up to four signers, but only the first is required. Note that these signer fields will not appear for local filers filing with Netfile.

### Cover Page, Part 2 ###

### Section 5: Officeholder or Candidate Controlled Committee ###

* The Filer committee type must be Candidate.
* Candidate must be linked as Candidate / Candidate of to the Filer, and must be marked as Primary for the Filer.
* Address is the primary address on the linked Candidate’s record.
* Office Sought or Office Held, Jurisdiction Description and District Number of the linked Candidate must be filled in.

### Section 5 (b): Related Committees Not Included in this Statement ###

* Related Committee must be a Political Committee linked to the Filer using the Authorized Committee/Authorized Committee of Relationship. There can be multiple committees.
* If the Controlled box is checked under Committee Info on the linked committee, the Yes box will be checked under Controlled Committee? section.

### Section 6: Primarily Formed Ballot Measure Committee ###

* First [create a Ballot Measure](https://ispolitical.com/how-do-i-add-a-ballot-measure/). Ensure the Letter or Number and Jurisdiction fields are filled out.
* Next link Ballot Measure with Ballot Measure Supported or Ballot Measure Opposed.

### Section 6(b): Identify the controlling officeholder, candidate, or state measure proponent, if any ###

* Filer committee type must be Candidate or Ballot Measure.
* Filer must have Controlled checked under Committee Info.
* Filer must have a linked Candidate using the Candidate/Candidate of Relationship.
* Address is the primary address on the linked Candidate’s record.
* Office Sought or Office Held and District Number of the linked Candidate must be filled in.

### Section 7: Primarily Formed Candidate/Officeholder Committee ###

* Candidate must be linked via either Opposed Candidate/Candidate Opposed By or Supported Candidate/Candidate Supported By relationship. 
* Linked Candidate must have Office Sought filled in.

### Summary Page ###

### Contributions Received ###

### Line 1 (Schedule A – Monetary Contributions) ###

* **Transaction Types:** Monetary Contributions (including contributions with tags Conduit-Conduit Check, Conduit-Donor Check, and Transfer), Loan Received Balance Reduction, Enforceable Pledge Payment, Enforceable Pledge (as memo), Anonymous Monetary Contributions (always unitemized)

### Line 2 (Schedule B-Part 1, Loans Received) ###

* **Transaction Type:** Loans Received

### Line 3, SUBTOTAL CASH CONTRIBUTIONS ###

* Total of Line 1 and Line 2

### Line 4 (Schedule C, Nonmonetary Contributions) ###

* **Transaction Types:** Inkind Contribution (including contributions with tags Conduit – Inkind), Accrued Expense Balance Reduction

### Line 5, TOTAL CONTRIBUTIONS RECEIVED ###

### Expenditures Made ###

### Line 6 (Schedule E, Payments Made) ###

* **Transaction Types:** Expenses, Accrued Expense Payment, Loan Made Balance Reduction, Non-Monetary Expense (as memo), Refunded Contribution, Allocation Transfer Made (from State to Federal), Loan Rec’d Interest Payment

### Line 7 (Schedule H, Loans Made to Others) ###

* **Transaction Type:** Loan Made

### Line 8, SUBTOTAL CASH PAYMENTS ###

* Total of Line 6 and Line 7

### Line 9 (Schedule F, Accrued Expenses) ###

* **Transaction Type:** Accrued Expense

### Line 10 (Schedule C, Nonmonetary Contributions) ###

* **Transaction Type:** Inkind Contribution (including contributions with tags Conduit – Inkind), Accrued Expense Balance Reduction

### Line 11, TOTAL EXPENDITURES MADE ###

* Total of Lines 8, 9, and 10

### Current Cash Statement ###

### Line 12, Beginning Cash Balance ###

* This is the total from Line 16 on the previous report

### Line 13, Cash Receipts ###

* Amount from Line 3 (Subtotal Cash Contributions) on main Summary Page

### Line 14 (Schedule I, Misc Increases to Cash) ###

* Transaction Type: Other Income, Anonymous Other Income (always unitemized), Allocation Transfer (Federal to State), Refunded Expense

### Line 15, Cash Payments ###

* Amount from Line 8 (Subtotal Cash Payments) on main Summary Page

### Line 16, ENDING CASH BALANCE ###

* Add Lines 12, 13, and 14, then subtract Line 15

### Line 17 (Schedule B-Part 2, Loan Guarantors) ###

* Transaction Type: Loan Received Guarantee

### Cash Equivalents and Outstanding Debts ###

### Line 18, Cash Equivalents ###

* Ending Investment Balance pulled from the criteria screen when generating the report

### Line 19, Outstanding Debts ###

* Total of Line 2 and Line 9 from Column B

### Line 22, Expenditure Ceiling Summary for State Candidates  
 ###

The following transactions are included in the Line 22 calculation:

###  ###

### NOT INCLUDED IN SUMMARY TOTALS ###

Schedule D, Summary of Expenditures Supporting/Opposing Others:

* **Transaction Type:** Support/Oppose Memos, Loan Made Balance Reduction

Schedule G, Payments Made by an Agent or Independent Contractor:

* **Transaction Type:** Expense (split), Accrued Expense (split), State Portion of Allocated Expense, State Portion of Allocated Accrued Expense Payment, State Portion of Allocated Non-Monetary Expense (as memo)
* In-kind Contributions, except from Party Committees
* Expenses, Accrued Expenses, Non-Monetary Expenses, and Refunded Expenses, except those with the Exempt from Expenditure Ceiling tag
* Sch G has a $500 itemization threshold.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[« What Is the CA 460 Report & How Do I Prepare and File It?](/What-Is-the-CA-46-Report-How-Do-I-Prepare-and-File-It)  
[Checkboxes for Sch D on CA 460 »](/Checkboxes-for-Sch-D-on-CA-46)