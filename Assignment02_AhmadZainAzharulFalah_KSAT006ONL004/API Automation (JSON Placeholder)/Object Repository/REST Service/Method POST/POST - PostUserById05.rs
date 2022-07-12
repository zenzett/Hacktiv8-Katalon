<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById05</name>
   <tag></tag>
   <elementGuidId>c783554b-6cb6-4543-9673-ecb12995c2d9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Awan\&quot;,\n  \&quot;body\&quot;: \&quot;Awan adalah massa yang dapat dilihat dari tetesan air atau kristal beku yang menggantung di atmosfer yang berada di atas permukaan bumi atau permukaan planet lain. Awan juga merupakan massa terlihat yang tertarik oleh gravitasi, seperti massa materi dalam ruang yang disebut awan antarbintang dan nebula. Awan dipelajari dalam ilmu awan atau fisika awan, suatu cabang meteorologi.\&quot;,\n  \&quot;userId\&quot;: \&quot;05\&quot;\n}&quot;,
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

WS.verifyElementPropertyValue(response, 'title', 'Awan')
WS.verifyElementPropertyValue(response, 'body', 'Awan adalah massa yang dapat dilihat dari tetesan air atau kristal beku yang menggantung di atmosfer yang berada di atas permukaan bumi atau permukaan planet lain. Awan juga merupakan massa terlihat yang tertarik oleh gravitasi, seperti massa materi dalam ruang yang disebut awan antarbintang dan nebula. Awan dipelajari dalam ilmu awan atau fisika awan, suatu cabang meteorologi.')
WS.verifyElementPropertyValue(response, 'userId', '05')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
