 API Sample Code: Javascript
==========

API submissions require very specific steps and language to send us data successfully. .

Below is sample Java Script code you can use as a sample of transmitting data via the API. However, please be aware that client side JavaScript is a security risk, as your API credentials could be stolen. We strongly recommend that you use server side code instead.

JavaScript/jQuery Sample  

----------

```
jQuery(“#submitButton”).click(function () {var authorizationString =  “Basic XXXXXXXXXXXXXXX” //replace this with your Base64 encoded credentialsvar submitData = new Object();submitData.lastname = jQuery(“#LastName”).val();submitData.firstname = jQuery(“#FirstName”).val();submitData.notes = jQuery(“#Notes”).val();submitData.source = window.location.href;submitData[“Emails”] = [{EmailAddress: jQuery(“#Email”).val();}];data = JSON.stringify(submitData);jQuery.ajax({type: “POST”,contentType: “application/json”,url: “https://app.ispolitical.com/api/PublicForms“,beforeSend: function (request) {request.setRequestHeader(“Authorization”,authorizationString);},data: data,dataType: “json”,success: function (result) {jQuery(“#SignupForm”).html(“<div class=’thankyou’>Thank you for your help on the campaign! We’ll be in touch shortly.</div>”);}});});
```

[Help File Home](/help/) | [Full Index](/Help-File-Directory/) | [Contact Support](mailto:support@ISPolitical.com)

[⇑ About ISP’s API](/About-ISP-s-API)  
[« How Do I Add Volunteers With Flags Via ISP’s API?](/How-Do-I-Add-Volunteers-With-Flags-Via-ISP-s-API)  
[Budget Category Request API »](/Budget-Category-Request-API)