<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get scan status</name>
   <tag></tag>
   <elementGuidId>52792880-1822-4cb8-9c07-fc10ac8c0f32</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA</value>
      <webElementGuid>e61e036b-6add-4d15-99ea-98bbc6c35399</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>08e186b5-5607-4d01-b0bb-6ec643f22331</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/doc/scan-status</restUrl>
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
      <id>dbb98b3a-a5a7-4cd2-a180-60e3e5dd4b77</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.queue_id</defaultValue>
      <description></description>
      <id>bd3fbe9f-fc95-411b-ba0c-bf9d103f47d5</id>
      <masked>false</masked>
      <name>queue_id</name>
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
WS.verifyElementPropertyValue(response, 'message', &quot;queue_id is required in the param request.&quot;)
WS.verifyElementPropertyValue(response, 'data', null)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
