<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get saved data</name>
   <tag></tag>
   <elementGuidId>b73b3e3c-0cf7-4222-93fa-51834ef9ce52</elementGuidId>
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
      <webElementGuid>126f770c-3399-4476-ae78-e3532596ad31</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fd3e2014-805e-4977-8f14-b65e644c81f4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/doc/getData?meta_data=[&#xd;
  {&#xd;
    &quot;date&quot;: &quot;13/2/2024&quot;,&#xd;
    &quot;block&quot;: &quot;lingasugur&quot;,&#xd;
    &quot;cluster&quot;: &quot;kannal&quot;,&#xd;
    &quot;gp_code&quot;: &quot;3384&quot;,&#xd;
    &quot;gp_name&quot;: &quot;Kannal&quot;,&#xd;
    &quot;school_no&quot;: &quot;23851&quot;,&#xd;
    &quot;school_name&quot;: &quot;GOVT.LOWER PRIMARY SCHOOL JAKKARAMADU&quot;&#xd;
  }&#xd;
]</restUrl>
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
      <id>c8d4f488-3630-4efe-a343-01a883dc6a2d</id>
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

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

WS.verifyElementPropertyValue(response, 'status', 200)
//WS.verifyElementPropertyValue(response, 'data[0].total', &quot;10&quot;)
//WS.verifyElementPropertyValue(response, 'data[0].student_no', &quot;1&quot;)
//WS.verifyElementPropertyValue(response, 'data[1].student_no', &quot;2&quot;)
////WS.verifyElementPropertyValue(response, 'data[1].student_no', &quot;11&quot;)
//
//WS.verifyElementPropertyValue(response, 'data[1].predictions', [])

WS.verifyElementPropertyValue(response, 'data[0].total', 2)
WS.verifyElementPropertyValue(response, 'data[0].gender', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].class_no', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data[0].student_no', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[0].tag', &quot;question_1_row_1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[1].tag', &quot;question_2_row_1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[2].tag', &quot;question_3_row_1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[0].text', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[1].text', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[2].text', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].predictions[3].text', &quot;1&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
