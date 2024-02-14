<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Export data as csv</name>
   <tag></tag>
   <elementGuidId>05f90f50-83c8-4445-9cd0-df05e7c78646</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;export_to\&quot;: \&quot;gdrive\&quot;,\n    \&quot;meta_data\&quot;: [\n        {\n            \&quot;date\&quot;: \&quot;\&quot;,\n            \&quot;block\&quot;: \&quot;bangalore\&quot;,\n            \&quot;cluster\&quot;: \&quot;bangalore-rural\&quot;,\n            \&quot;gp_code\&quot;: \&quot;123\&quot;,\n            \&quot;gp_name\&quot;: \&quot;bangalore\&quot;,\n            \&quot;school_no\&quot;: \&quot;002\&quot;\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA</value>
      <webElementGuid>95389b76-9c27-467c-9f78-ea8ed134b432</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>99c85d57-2993-44e9-b684-10de8902a414</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${URL}/doc/export</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.devURL</defaultValue>
      <description></description>
      <id>55133dae-9a02-4925-bd2a-55b6e10b0746</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'status', 400)
WS.verifyElementPropertyValue(response, 'message', &quot;No records found&quot;)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
