<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById07</name>
   <tag></tag>
   <elementGuidId>37f5efab-4307-4fe7-8234-e1015310543b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Api\&quot;,\n  \&quot;body\&quot;: \&quot;Api adalah oksidasi cepat suatu bahan (bahan bakar) dalam proses kimia eksotermik dari pembakaran, yang mengakibatkan pelepasan panas, cahaya, dan berbagai produk reaksi. Panas yang dihasilkan api disebabkan oleh perubahan ikatan rangkap lemah dalam molekul oksigen, O2, menjadi ikatan yang lebih kuat, menghasilkan karbon dioksida dan air, serta melepaskan energi (418 kJ per 32 g O2); energi ikatan bahan bakar sebenarnya hanya memainkan peran kecil di sini. Pada titik tertentu dalam reaksi pembakaran akan muncul nyala api, yang disebut titik pengapian. Nyala api adalah bagian api yang terlihat. Api terutama terdiri dari karbon dioksida, uap air, oksigen dan nitrogen. Jika cukup panas, gas bisa terionisasi untuk menghasilkan plasma. Tergantung pada zat yang menyala, dan zat lain yang ikut tercampur, warna nyala api dan intensitas api bisa berbeda-beda.\&quot;,\n  \&quot;userId\&quot;: \&quot;07\&quot;\n}&quot;,
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

WS.verifyElementPropertyValue(response, 'title', 'Api')
WS.verifyElementPropertyValue(response, 'body', 'Api adalah oksidasi cepat suatu bahan (bahan bakar) dalam proses kimia eksotermik dari pembakaran, yang mengakibatkan pelepasan panas, cahaya, dan berbagai produk reaksi. Panas yang dihasilkan api disebabkan oleh perubahan ikatan rangkap lemah dalam molekul oksigen, O2, menjadi ikatan yang lebih kuat, menghasilkan karbon dioksida dan air, serta melepaskan energi (418 kJ per 32 g O2); energi ikatan bahan bakar sebenarnya hanya memainkan peran kecil di sini. Pada titik tertentu dalam reaksi pembakaran akan muncul nyala api, yang disebut titik pengapian. Nyala api adalah bagian api yang terlihat. Api terutama terdiri dari karbon dioksida, uap air, oksigen dan nitrogen. Jika cukup panas, gas bisa terionisasi untuk menghasilkan plasma. Tergantung pada zat yang menyala, dan zat lain yang ikut tercampur, warna nyala api dan intensitas api bisa berbeda-beda.')
WS.verifyElementPropertyValue(response, 'userId', '07')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
