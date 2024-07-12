 How Do I Locate Uncontacted Individuals Using Advanced Search?
==========

If you’re looking for potential donors who have not had a Communication added to their record yet, you can do so via Advanced Search.

1. Click on the Add/Search menu, then Advanced Search.  
2. Your first rule is going to narrow this down to Individuals.

* Area: Entities
* Field: Entity Type
* Relationship: Is Any Of
* Value: Individual

3. Click Add Rule.  
4. Now, we need to exclude individuals with at least one Communication on their record.

* Area: Communications
* Field: Count
* Relationship: Is More Than
* Value: 0

5. Click Add as Exclusion.  
6. Click Run Search.

If you’d like to pull this list but don’t want to exclude all Communication Types, use this search instead.

1. Narrow it down to Individuals.

* Area: Entities
* Field: Entity Type
* Relationship: Is Any Of
* Value: Individuals

2. Click Add Rule.  
3. Now, we need to exclude all the Communication Types you’d like to exclude.

* Area: Communications
* Field: Communication Type
* Relationship Is Any Of
* Value: [insert any Communication Type you’d like to exclude]

4. Click Add as Exclusion.  
5. Click Run Search.

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ What Is Advanced Search and How Do I Use It?](/What-Is-Advanced-Search-and-How-Do-I-Use-It)  
[« What Is the Difference Between Actor and User Fields in Advanced Search?](/Difference-Between-Actor-and-User-Fields-in-Advanced-Search)  
[About Reports in ISP »](/About-Reports-in-ISP)