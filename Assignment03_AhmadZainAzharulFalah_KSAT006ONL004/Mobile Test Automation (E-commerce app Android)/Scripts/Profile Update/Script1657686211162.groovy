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

Mobile.startApplication('C:\\Users\\095054\\Documents\\Workspace Hacktiv8-Katalon\\Assignment03_AhmadZainAzharulFalah_KSAT006ONL004\\Mobile Test Automation (E-commerce app Android)\\APK\\ECommerce -SAMPLE-Android.apk', 
    true)

Mobile.tap(findTestObject('Record Capture/Menu/android.widget.FrameLayout - Profile'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.TextView - EDIT'), 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.RelativeLayout - Name'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.EditText - Your Name'), 0)

Mobile.clearText(findTestObject('Record Capture/Profile/android.widget.EditText - Your Name'), 0)

Mobile.setText(findTestObject('Object Repository/Record Capture/Profile/android.widget.EditText'), 'Zain', 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.Button - OK'), 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.RelativeLayout - Email'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.EditText - your.emailgmail.com'), 0)

Mobile.clearText(findTestObject('Record Capture/Profile/android.widget.EditText - your.emailgmail.com'), 0)

Mobile.setText(findTestObject('Record Capture/Profile/android.widget.EditText'), 'zain@gmail.com', 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.Button - OK'), 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.RelativeLayout - Number'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.EditText - 628123456789'), 0)

Mobile.clearText(findTestObject('Record Capture/Profile/android.widget.EditText - 628123456789'), 0)

Mobile.setText(findTestObject('Record Capture/Profile/android.widget.EditText'), '081201010101', 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.Button - OK'), 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.RelativeLayout - Address'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Profile/android.widget.EditText - Your Address'), 0)

Mobile.clearText(findTestObject('Record Capture/Profile/android.widget.EditText - Your Address'), 0)

Mobile.setText(findTestObject('Record Capture/Profile/android.widget.EditText'), 'Depok', 0)

Mobile.tap(findTestObject('Record Capture/Profile/android.widget.Button - OK'), 0)

Mobile.pressBack()

