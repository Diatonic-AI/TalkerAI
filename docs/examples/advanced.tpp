// Advanced Talk++ Example
// Demonstrates conditional logic, assignments, and service integrations

user_email: "user@example.com"
notification_service: "Twilio"

if file uploaded to S3 and file size > 1000000
then process image using ImageMagick
then store metadata in PostgreSQL 
then send notification using notification_service
else
then store original file
then log file size

when order status changes to "shipped"
then send tracking email using SendGrid
then update inventory count
then trigger analytics event 