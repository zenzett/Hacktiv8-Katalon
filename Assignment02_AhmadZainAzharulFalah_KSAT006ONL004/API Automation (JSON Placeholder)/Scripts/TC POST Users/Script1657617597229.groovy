import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById01'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById02'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById03'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById04'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById05'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById06'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById07'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById08'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById09'))

WS.sendRequestAndVerify(findTestObject('REST Service/Method POST/POST - PostUserById10'))

