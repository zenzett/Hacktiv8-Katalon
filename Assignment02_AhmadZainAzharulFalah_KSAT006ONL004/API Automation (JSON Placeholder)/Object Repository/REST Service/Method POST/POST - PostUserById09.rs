<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById09</name>
   <tag></tag>
   <elementGuidId>67439044-4a90-4d6e-84c0-45d28affa6e2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Tanah\&quot;,\n  \&quot;body\&quot;: \&quot;Tanah (bahasa Yunani: pedon; bahasa Latin: solum) adalah bagian kerak bumi yang tersusun dari mineral dan bahan organik. Tanah sangat vital peranannya bagi semua kehidupan di bumi karena tanah mendukung kehidupan tumbuhan dengan menyediakan unsur hara dan air sekaligus sebagai penopang akar. Struktur tanah yang berongga-rongga juga menjadi tempat yang baik bagi akar untuk bernapas dan tumbuh. Tanah juga menjadi habitat hidup berbagai mikroorganisme. Bagi sebagian besar hewan darat, tanah menjadi lahan untuk hidup dan bergerak.\&quot;,\n  \&quot;userId\&quot;: \&quot;09\&quot;\n}&quot;,
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

WS.verifyElementPropertyValue(response, 'title', 'Tanah')
WS.verifyElementPropertyValue(response, 'body', 'Tanah (bahasa Yunani: pedon; bahasa Latin: solum) adalah bagian kerak bumi yang tersusun dari mineral dan bahan organik. Tanah sangat vital peranannya bagi semua kehidupan di bumi karena tanah mendukung kehidupan tumbuhan dengan menyediakan unsur hara dan air sekaligus sebagai penopang akar. Struktur tanah yang berongga-rongga juga menjadi tempat yang baik bagi akar untuk bernapas dan tumbuh. Tanah juga menjadi habitat hidup berbagai mikroorganisme. Bagi sebagian besar hewan darat, tanah menjadi lahan untuk hidup dan bergerak.')
WS.verifyElementPropertyValue(response, 'userId', '09')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
