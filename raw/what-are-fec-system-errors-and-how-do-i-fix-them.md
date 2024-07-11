 What Are FEC System Errors and How Do I Fix Them?
==========

Applies To: Federal Election Commission

Sometimes the FEC system goes down or errors out when accepting a report. First, read the error. They’re not always clearly written but can have useful information. These errors are different than those in ISP when generating the report.

If you’re still unsure or having trouble, a screenshot is always a good idea to demonstrate best efforts to file. This will come in handy if you need to have a discussion with folks at the FEC or with us.

Finally, remember, it can take several hours for reports to show up on the FEC website. If yours doesn’t show up by the morning after your attempt, at that point it’s worth contacting your analyst with the FEC.

**SYSTEM ERROR: SOMETHING WENT WRONG, PLEASE CONTACT ELECTRONIC FILING OFFICE:** Your best bet is to try filing again. In fact, filing again is always a great way to go. 

**FILING ALREADY IN PROCESS!!!! POSSIBLE DUPLICATE SUBMISSION:** If you see this message, they have received your filing.

**Agency Response: fparse returned with error.:** This is an internal FEC error directly from the agency. Please try filing again and if you get the possible duplicate message above, your first report was in fact submitted.

These errors may occur when uploading your report.

Bad Format Messages 

----------

**BADFORMAT Candidate ID H########/S#######P####### does not exist:** When submitting a filing for a Form 2, the Candidate ID number cannot be identified.

**BADFORMAT Cannot find superseded filing:** When submitting an amendment, the original filing cannot be located in the system database.

**BADFORMAT Filer ID FEC-##### does not exist for C########:** When submitting an amendment, the original filing ID could not be found for the committee. The original filing ID can be found by going to [http://www.fec.gov/finance/disclosure/efile\_search.shtml](http://www.fec.gov/finance/disclosure/efile_search.shtml). The ID must be from the original report and not the latest amendment. It also must be entered exactly as it appears on the website (it is case-sensitive).

**BADFORMAT Filer ID not specified:** When submitting an amendment, no original Filing ID was specified. The original filing ID can be found by going to [http://www.fec.gov/finance/disclosure/efile\_search.shtml](http://www.fec.gov/finance/disclosure/efile_search.shtml). The ID must be from the original report and not the latest amendment. It also must be entered exactly as it appears on the website, as it is case-sensitive.

**BADFORMAT Filer Password not specified:** When submitting a filing, no password was submitted along with the filing, or the password was not in the correct format. The password must be entered in the proper case, exactly as it was issued.

**BADFORMAT Filing did not pass validation:** When submitting a filing, the filing failed the validation routine and the results will be sent via email or fax.

**BADFORMAT Filing to be amended does NOT belong to you:** When submitting an amendment, the original filing ID does not belong to the committee filing.

**BADFORMAT Filing to be amended is not a Form 2:** When submitting an amendment for a Form 2, the original filing ID is not from Form 2.

**BADFORMAT Invalid fax number (You must specify all 10 digits):** When submitting a filing, the fax number must contain all 10 digits.

**BADFORMAT Invalid Superseded specification:** When submitting an amendment, the original filing ID is not in the correct format FEC-#####. You can find your original filing ID by looking up your committee on the FEC’s site and navigating to the Filings tab.

**BADFORMAT Invalid Superseded specification Back Door:** When submitting an amendment, the original filing ID is not in the correct format FEC-#####.

**BADFORMAT Unrecognized header:** The header line of the submitted filing is not in the correct format. This applies to users of vendor software or filers who have created their own comma-delimited format file.

**BADFORMAT You must specify FilerFAX and/or FilerEmail:** When submitting a filing, a FAX and/or email are necessary for Processing. Our best recommendations for when you receive this error are to log out and back in again, wait a few hours, or contact us to attempt a manual filing through the FEC’s portal on your behalf.

Error Messages

----------

**ERROR: Amended filing MUST amend the original filing NOT another amendment:** When submitting an amendment, the original filing ID belongs to another amendment. An amendment must amend the original report.

**ERROR: Inconsistent report IDs:** The header line in the filing being submitted reflects a different filing ID than the filing sent/entered by the uploading utility.

**ERROR: Inconsistent report numbers. This filing is older than the last amendment:** The filing being sent has a sequence number lower than the last posted filing for this report. Usually this is a filing that should have an amendment number but is being sent as a new report. It is also possible that an incorrect original report number has been entered. Ensure that the correct original report number has been entered and then regenerate the report.

**ERROR: Incorrect Date Length:** A date in the filing being submitted is in the incorrect format. The correct format is CCYYMMDD and can only be numeric values.

**ERROR: Non Numeric Value For Date:** A date in the filing being submitted is in the incorrect format. The correct format is CCYYMMDD and can only be numeric values.

**ERROR: Password Is Not Valid For C########:** The password being submitted is either not in the right case or is not valid for the committee.

**ERROR: You can only file a Form 6!!:** The filing password assigned to the committee only allows the Committee to file Forms 6.

General Upload Messages, Success and Error

----------

**Back Door System Busy Please Try Again Later. If this problem persists, Call Tech Support:** The back door routine is run prior to the validation routine and is too busy to accept filings at this time. Please try again later.

**Filing Accepted and Is Being Processed:** The filing submitted has been accepted and is being processed. A confirmation email/fax should be sent shortly thereafter.

**Filing Accepted and Is FEC-#####:** The filing submitted has been accepted and processed. The Filing ID receipt is FEC-#####.

**Filing Accepted For Processing:** The filing submitted has been accepted and is being processed. If you do not receive a receipt, call the Electronic Filing Office.

**FILING ALREADY POSTED!!! Filing FEC-%d has the same MD5 total:** The filing being submitted has already been uploaded and posted. They are exact duplicates.

**Filing Failed Validation:** When submitting a filing, the filing failed the validation routine and the results will be sent via email or fax.

**Invalid FEC CSV Version Number!! Please Upgrade to The Most Current Version:** The filing is being submitted in an older file format version that is no longer acceptable for filing electronically. Please upgrade to the most current version by contacting the software vendor.

**INVALID Header:** The header line of the submitted filing is not in the correct format. This applies to users of vendor software or filers who have created their own comma-delimited format file.

**MD5 Not working correctly Aborting Upload (Please contact Electronic Filing Office):** Internal error; Please contact the Electronic Filing Office.

**Election Code missing: ?:** The transaction in question is missing an election (primary, general, special, or runoff). Add an election to the transaction. If you used Non-Election as the election, the election field on the report will be blank, which is expected, and the warning can be ignored.

**Submitting and Filer Committee IDs Inconsistent:** The Committee ID in the upload routine and the ID in the file itself are not the same.

**System Busy Please try Again Later. If this problem persists Call Tech Support:** When submitting a filing, the filing is rejected due to a high volume of filings. Please try again later.

**System Busy!! Please Try Again Later or Contact The Electronic filing Office For Assistance:** When submitting a filing, the filing is rejected due to a high volume of filings. Please try again later.

**You Can NOT Amend a NEW report!:** The filing being submitted reflects in the header line that it is a new report, but also contains an original Filing ID in the Original amendment Filing ID field. Please contact the Electronic Filing Office.

**You must enter the report you are amending!!:** The filing being submitted reflects in the header line that it is an amendment, but does not contain a Filing ID in the Original Amendment Filing ID.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[« About Report Post-Generation](/About-Report-Post-Generation)  
[Wrong FEC Password Error Troubleshooting »](/Wrong-FEC-Password-Error-Troubleshooting)