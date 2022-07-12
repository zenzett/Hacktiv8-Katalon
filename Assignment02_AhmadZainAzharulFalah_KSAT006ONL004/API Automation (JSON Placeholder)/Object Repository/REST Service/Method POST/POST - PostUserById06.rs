<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById06</name>
   <tag></tag>
   <elementGuidId>741e4a0d-e6e3-47e6-a350-ac5cecac3441</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Udara\&quot;,\n  \&quot;body\&quot;: \&quot;Udara merujuk kepada campuran gas yang terdapat pada permukaan bumi. Udara tidak tampak mata, tidak berbau, dan tidak ada rasanya. Kehadiran udara hanya dapat dilihat dari adanya angin yang menggerakan benda. Udara termasuk salah satu jenis sumber daya alam karena memiliki banyak fungsi bagi makhluk hidup. Kandungan elemen senyawa gas dan partikel dalam udara akan berubah-ubah dengan ketinggian dari permukaan tanah. Demikian juga massanya, akan berkurang seiring dengan ketinggian. Semakin dekat dengan lapisan troposfer, maka udara semakin tipis, sehingga melewati batas gravitasi bumi, maka udara akan hampa sama sekali. Apabila makhluk hidup bernapas, kandungan oksigen berkurang, sementara kandungan karbon dioksida bertambah. Ketika tumbuhan menjalani sistem fotosintesa, oksigen kembali dibebaskan.\&quot;,\n  \&quot;userId\&quot;: \&quot;06\&quot;\n}&quot;,
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

WS.verifyElementPropertyValue(response, 'title', 'Udara')
WS.verifyElementPropertyValue(response, 'body', 'Udara merujuk kepada campuran gas yang terdapat pada permukaan bumi. Udara tidak tampak mata, tidak berbau, dan tidak ada rasanya. Kehadiran udara hanya dapat dilihat dari adanya angin yang menggerakan benda. Udara termasuk salah satu jenis sumber daya alam karena memiliki banyak fungsi bagi makhluk hidup. Kandungan elemen senyawa gas dan partikel dalam udara akan berubah-ubah dengan ketinggian dari permukaan tanah. Demikian juga massanya, akan berkurang seiring dengan ketinggian. Semakin dekat dengan lapisan troposfer, maka udara semakin tipis, sehingga melewati batas gravitasi bumi, maka udara akan hampa sama sekali. Apabila makhluk hidup bernapas, kandungan oksigen berkurang, sementara kandungan karbon dioksida bertambah. Ketika tumbuhan menjalani sistem fotosintesa, oksigen kembali dibebaskan.')
WS.verifyElementPropertyValue(response, 'userId', '06')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
