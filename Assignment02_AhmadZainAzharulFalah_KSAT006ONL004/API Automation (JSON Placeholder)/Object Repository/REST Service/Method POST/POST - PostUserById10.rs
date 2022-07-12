<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById10</name>
   <tag></tag>
   <elementGuidId>7ac54eff-6719-4e78-a405-efb2ab84c2b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Pasir\&quot;,\n  \&quot;body\&quot;: \&quot;Pasir adalah material butiran yang terdiri dari partikel batuan dan mineral yang terpecah halus. Ukuran pasir pasir lebih halus dari kerikil dan lebih kasar dari lanau. Pasir juga bisa mengacu pada suatu kelas tekstur dari tanah atau jenis tanah; yaitu, tanah yang mengandung lebih dari 85 persen partikel berukuran pasir berdasarkan massa.\&quot;,\n  \&quot;userId\&quot;: \&quot;10\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>dacd228a-e4db-4827-9b8e-249c217333ef</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'title', 'Pasir')
WS.verifyElementPropertyValue(response, 'body', 'Pasir adalah material butiran yang terdiri dari partikel batuan dan mineral yang terpecah halus. Ukuran pasir pasir lebih halus dari kerikil dan lebih kasar dari lanau. Pasir juga bisa mengacu pada suatu kelas tekstur dari tanah atau jenis tanah; yaitu, tanah yang mengandung lebih dari 85 persen partikel berukuran pasir berdasarkan massa.')
WS.verifyElementPropertyValue(response, 'userId', '10')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
