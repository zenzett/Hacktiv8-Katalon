<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - PostUserById08</name>
   <tag></tag>
   <elementGuidId>4cdd7bba-dfd8-4658-862a-ee6530819170</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;Pohon\&quot;,\n  \&quot;body\&quot;: \&quot;Dalam botani, pohon adalah tumbuhan menahun dengan batang yang tumbuh memanjang, mendukung cabang dan daun pada sebagian besar spesies. Dalam beberapa penggunaan, definisi pohon mungkin lebih sempit, biasanya hanya mengacu pada tanaman berkayu dengan pertumbuhan sekunder, tanaman yang dapat digunakan sebagai kayu, atau tanaman yang tumbuh hingga ketinggian tertentu. Dalam definisi yang lebih luas, palem, pakis, pisang, dan bambu juga termasuk jenis pohon. Pohon bukanlah kelompok taksonomi tetapi mencakup berbagai spesies tumbuhan yang mengembangkan batang dan cabang sebagai cara untuk menjulang di atas tumbuhan lain demi bersaing mendapatkan sinar matahari. Pohon cenderung berumur panjang, beberapa pohon bisa hidup hingga beberapa ribu tahun. Pohon telah tumbuh di Bumi setidaknya selama 370 juta tahun. Diperkirakan terdapat sekitar tiga triliun pohon dewasa di dunia.\&quot;,\n  \&quot;userId\&quot;: \&quot;08\&quot;\n}&quot;,
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

WS.verifyElementPropertyValue(response, 'title', 'Pohon')
WS.verifyElementPropertyValue(response, 'body', 'Dalam botani, pohon adalah tumbuhan menahun dengan batang yang tumbuh memanjang, mendukung cabang dan daun pada sebagian besar spesies. Dalam beberapa penggunaan, definisi pohon mungkin lebih sempit, biasanya hanya mengacu pada tanaman berkayu dengan pertumbuhan sekunder, tanaman yang dapat digunakan sebagai kayu, atau tanaman yang tumbuh hingga ketinggian tertentu. Dalam definisi yang lebih luas, palem, pakis, pisang, dan bambu juga termasuk jenis pohon. Pohon bukanlah kelompok taksonomi tetapi mencakup berbagai spesies tumbuhan yang mengembangkan batang dan cabang sebagai cara untuk menjulang di atas tumbuhan lain demi bersaing mendapatkan sinar matahari. Pohon cenderung berumur panjang, beberapa pohon bisa hidup hingga beberapa ribu tahun. Pohon telah tumbuh di Bumi setidaknya selama 370 juta tahun. Diperkirakan terdapat sekitar tiga triliun pohon dewasa di dunia.')
WS.verifyElementPropertyValue(response, 'userId', '08')
WS.verifyElementPropertyValue(response, 'id', '101')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
