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

Mobile.tap(findTestObject('Record Capture/Menu/android.widget.FrameLayout - Category'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.RelativeLayout - CategFashion'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.RelativeLayout - Batik'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - ADD TO CART'), 0)

Mobile.setText(findTestObject('Object Repository/Record Capture/Category/android.widget.EditText'), '3', 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - Add'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Menu/android.widget.ImageButton - Back'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.RelativeLayout - SKMEI Watch'), 0)

Mobile.tap(findTestObject('Record Capture/Category/android.widget.Button - ADD TO CART'), 0)

Mobile.setText(findTestObject('Record Capture/Category/android.widget.EditText'), '1', 0)

Mobile.tap(findTestObject('Record Capture/Category/android.widget.Button - Add'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Menu/android.widget.TextView - Cart'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - Checkout'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - PROCESS CHECKOUT'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - Yes'), 0)

Mobile.tap(findTestObject('Object Repository/Record Capture/Category/android.widget.Button - OK'), 0)

