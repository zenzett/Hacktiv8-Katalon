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

WebUI.callTestCase(findTestCase('Assignment/Login Only'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Capture Manual/CURA/Make Appointment/drop_facility'), 'Hongkong CURA Healthcare Center', 
    true)

WebUI.click(findTestObject('Capture Manual/CURA/Make Appointment/chkbox_apply_for_hospital_readmission'))

WebUI.click(findTestObject('Capture Manual/CURA/Make Appointment/radio_medicaid_programs'))

WebUI.setText(findTestObject('Capture Manual/CURA/Make Appointment/txt_visit_date_(required)'), '11/06/2022')

WebUI.setText(findTestObject('Capture Manual/CURA/Make Appointment/textarea_comment'), 'Jangan banyak nanya dah.')

WebUI.click(findTestObject('Capture Manual/CURA/Make Appointment/btn_book_appointment'))

WebUI.verifyElementText(findTestObject('Capture Manual/CURA/Appointment Confirmation/txt_Facility'), 'Hongkong CURA Healthcare Center')

WebUI.verifyElementText(findTestObject('Capture Manual/CURA/Appointment Confirmation/txt_Readmission'), 'Yes')

WebUI.verifyElementText(findTestObject('Capture Manual/CURA/Appointment Confirmation/txt_Program'), 'Medicaid')

WebUI.verifyElementText(findTestObject('Capture Manual/CURA/Appointment Confirmation/txt_Visit Date'), '11/06/2022')

WebUI.verifyElementText(findTestObject('Capture Manual/CURA/Appointment Confirmation/txt_Comment'), 'Jangan banyak nanya dah.')

WebUI.click(findTestObject('Capture Manual/CURA/Appointment Confirmation/btn_Go to Homepage'))

WebUI.click(findTestObject('Capture Manual/CURA/Home/menu_Home'))

WebUI.click(findTestObject('Capture Manual/CURA/Home/btn_Logout'))

WebUI.closeBrowser()

